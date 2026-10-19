//! Protocol regression: asleep BUSY never goes low until an NSS-only wake pulse.
use super::*;
use core::{cell::RefCell, convert::Infallible};
use embedded_hal::{digital, spi};
use embedded_hal_async::spi::Operation;
use std::{rc::Rc, vec::Vec};

#[derive(Default)]
struct Model {
    cs_high: bool,
    asleep: bool,
    waking: bool,
    stuck: bool,
    events: Vec<u16>,
}
#[derive(Clone)]
struct Pin(Rc<RefCell<Model>>);
struct Spi(Rc<RefCell<Model>>);
impl digital::ErrorType for Pin {
    type Error = Infallible;
}
impl digital::InputPin for Pin {
    fn is_high(&mut self) -> Result<bool, Infallible> {
        let m = self.0.borrow();
        Ok(m.asleep || m.waking || m.stuck)
    }
    fn is_low(&mut self) -> Result<bool, Infallible> {
        Ok(!self.is_high()?)
    }
}
impl digital::OutputPin for Pin {
    fn set_low(&mut self) -> Result<(), Infallible> {
        self.0.borrow_mut().cs_high = false;
        Ok(())
    }
    fn set_high(&mut self) -> Result<(), Infallible> {
        self.0.borrow_mut().cs_high = true;
        Ok(())
    }
}
impl embedded_hal_async::digital::Wait for Pin {
    async fn wait_for_low(&mut self) -> Result<(), Infallible> {
        let pending = {
            let mut m = self.0.borrow_mut();
            if m.waking {
                m.waking = false;
                m.events.push(0);
            }
            m.asleep || m.stuck
        };
        if pending {
            core::future::pending::<()>().await;
        }
        Ok(())
    }
    async fn wait_for_high(&mut self) -> Result<(), Infallible> {
        core::future::pending().await
    }
    async fn wait_for_rising_edge(&mut self) -> Result<(), Infallible> {
        core::future::pending().await
    }
    async fn wait_for_falling_edge(&mut self) -> Result<(), Infallible> {
        core::future::pending().await
    }
    async fn wait_for_any_edge(&mut self) -> Result<(), Infallible> {
        core::future::pending().await
    }
}
impl spi::ErrorType for Spi {
    type Error = Infallible;
}
impl embedded_hal_async::spi::SpiDevice for Spi {
    async fn transaction(&mut self, ops: &mut [Operation<'_, u8>]) -> Result<(), Infallible> {
        let mut m = self.0.borrow_mut();
        if let [Operation::DelayNs(ns)] = ops {
            assert!(*ns >= 100_000);
            assert!(m.asleep);
            m.asleep = false;
            m.waking = true;
            m.events.push(0xffff);
            return Ok(());
        }
        assert!(!m.asleep && !m.waking, "command sent before wake BUSY-low");
        if let Operation::Transfer(status, command) = &mut ops[0] {
            if command.len() == 1 {
                // GET_VERSION response phase during backend initialization.
                status.fill(0x04);
                if let Operation::Read(response) = &mut ops[1] {
                    response.copy_from_slice(&[0, 3, 1, 1]);
                }
                return Ok(());
            }
            let opcode = u16::from_be_bytes([command[0], command[1]]);
            m.events.push(opcode);
            status.fill(0x04);
            if opcode == 0x0116 {
                assert!(
                    matches!(&ops[1], Operation::Write([0])),
                    "E80 must use LF RC"
                );
            }
            if opcode == 0x011b {
                assert!(matches!(&ops[1], Operation::Write([1, 0, 0, 0, 0])));
                m.asleep = true;
            }
        }
        Ok(())
    }
}

#[test]
fn initialization_selects_internal_lf_clock() {
    embassy_futures::block_on(async {
        let (mut driver, model) = fixture();
        driver.backend.initialize().await.unwrap();
        assert!(model.borrow().events.contains(&0x0116));
    });
}

// The NSS-only wake must not clock the SPI bus at all.
impl spi::SpiBus for Spi {
    fn read(&mut self, _: &mut [u8]) -> Result<(), Infallible> {
        panic!("unexpected SPI clocks")
    }
    fn write(&mut self, _: &[u8]) -> Result<(), Infallible> {
        panic!("unexpected SPI clocks")
    }
    fn transfer(&mut self, _: &mut [u8], _: &[u8]) -> Result<(), Infallible> {
        panic!("unexpected SPI clocks")
    }
    fn transfer_in_place(&mut self, _: &mut [u8]) -> Result<(), Infallible> {
        panic!("unexpected SPI clocks")
    }
    fn flush(&mut self) -> Result<(), Infallible> {
        Ok(())
    }
}

#[test]
fn nss_pulse_releases_chip_select_on_completion_and_cancellation() {
    use embedded_hal_async::spi::SpiDevice;
    embassy_futures::block_on(async {
        let model = Rc::new(RefCell::new(Model::default()));
        let mut device = ManualCsSpiDevice::new(Spi(model.clone()), Pin(model.clone()));
        let start = Instant::now();
        device
            .transaction(&mut [Operation::DelayNs(100_000)])
            .await
            .unwrap();
        assert!(start.elapsed() >= Duration::from_micros(100));
        assert!(model.borrow().cs_high);
        assert!(embassy_time::with_timeout(
            Duration::from_millis(1),
            device.transaction(&mut [Operation::DelayNs(100_000_000)])
        )
        .await
        .is_err());
        assert!(model.borrow().cs_high);
    });
}

fn fixture() -> (RadioDriver<Spi, Pin, Pin, Pin>, Rc<RefCell<Model>>) {
    let model = Rc::new(RefCell::new(Model::default()));
    let pin = Pin(model.clone());
    let mut driver = RadioDriver::new(Spi(model.clone()), pin.clone(), pin.clone(), pin);
    driver.initialized = true;
    (driver, model)
}

#[test]
fn retained_sleep_and_wake_with_busy_high_and_no_reset() {
    embassy_futures::block_on(async {
        let (mut driver, model) = fixture();
        for _ in 0..3 {
            driver.sleep().await.unwrap();
            assert_eq!(driver.state(), RadioState::Sleep);
            assert!(model.borrow().asleep);
            driver.wake().await.unwrap();
            assert_eq!(driver.state(), RadioState::Standby);
        }
        assert_eq!(model.borrow().events, [0x011b, 0xffff, 0, 0x011c].repeat(3));
        assert_eq!(driver.stats().resets, 0);
    });
}

#[test]
fn wake_timeout_does_not_send_standby_or_report_awake() {
    embassy_futures::block_on(async {
        let (mut driver, model) = fixture();
        driver.sleep().await.unwrap();
        model.borrow_mut().stuck = true;
        driver.timing.command_timeout = Duration::from_millis(2);
        assert_eq!(driver.wake().await, Err(Error::WakeUp));
        assert_eq!(driver.state(), RadioState::Sleep);
        assert!(!model.borrow().events.contains(&0x011c));
    });
}

#[test]
fn cancelled_standby_cleanup_can_be_retried_before_sleep() {
    embassy_futures::block_on(async {
        let (mut driver, model) = fixture();
        driver.state = RadioState::Rx;
        driver.operation_in_flight = true;
        model.borrow_mut().stuck = true;
        assert!(
            embassy_time::with_timeout(Duration::from_millis(2), driver.finish_standby())
                .await
                .is_err()
        );
        assert!(driver.operation_in_flight);
        model.borrow_mut().stuck = false;
        driver.sleep().await.unwrap();
        assert_eq!(driver.state(), RadioState::Sleep);
        assert!(!driver.operation_in_flight);
    });
}
