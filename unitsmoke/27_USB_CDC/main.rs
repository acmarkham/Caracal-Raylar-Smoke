// USB CDC serial smoke test.
//
// Activity: enumerates as a USB CDC-ACM serial port over the USB-C connector
// and sends "hello world\n" once per second after the host opens the port.

#![no_std]
#![no_main]

use defmt::{info, unwrap, warn};
use embassy_executor::Spawner;
use embassy_stm32::gpio::{Input, Output, Pull};
use embassy_stm32::peripherals::{PA9, USB_OTG_HS};
use embassy_stm32::rcc::*;
use embassy_stm32::time::mhz;
use embassy_stm32::usb::{Config as UsbDriverConfig, Driver as UsbDriver};
use embassy_stm32::Peri;
use embassy_time::Timer;
use embassy_usb::class::cdc_acm::{CdcAcmClass, State, USB_CLASS_CDC};
use embassy_usb::{Builder, Config as UsbConfig, UsbDevice};
use raylar_board_v1p0::{Board, Irqs, UsbCdc};
use {defmt_rtt as _, panic_probe as _};

const USB_VID: u16 = 0x1209;
const USB_PID: u16 = 0x0001;
const CDC_MAX_PACKET_SIZE: u16 = 64;

type UsbCdcDriver = UsbDriver<'static, USB_OTG_HS>;

#[embassy_executor::main]
async fn main(spawner: Spawner) -> ! {
    let mut config = embassy_stm32::Config::default();

    config.rcc.hse = Some(Hse {
        freq: mhz(16),
        mode: HseMode::Oscillator,
    });

    config.rcc.pll1 = Some(Pll {
        source: PllSource::HSE,
        prediv: PllPreDiv::DIV1,
        mul: PllMul::MUL10,
        divp: Some(PllDiv::DIV1),
        divq: Some(PllDiv::DIV2),
        divr: Some(PllDiv::DIV2),
    });

    config.rcc.sys = Sysclk::PLL1_R;

    let p = embassy_stm32::init(config);
    let Board { usb_cdc, leds, .. } = Board::new(p);

    info!("USB CDC smoke test started");
    usb_cdc_hello(spawner, usb_cdc, leds.sys_main_green).await
}

async fn usb_cdc_hello(
    spawner: Spawner,
    usb_cdc: UsbCdc<'static>,
    mut activity_led: Output<'static>,
) -> ! {
    let UsbCdc { usb, dm, dp, vbus } = usb_cdc;
    log_vbus(vbus);

    let ep_out_buffer = unwrap!(cortex_m::singleton!(: [u8; 256] = [0; 256]));
    let config_descriptor = unwrap!(cortex_m::singleton!(: [u8; 256] = [0; 256]));
    let bos_descriptor = unwrap!(cortex_m::singleton!(: [u8; 256] = [0; 256]));
    let msos_descriptor = unwrap!(cortex_m::singleton!(: [u8; 128] = [0; 128]));
    let control_buf = unwrap!(cortex_m::singleton!(: [u8; 64] = [0; 64]));
    let cdc_state = unwrap!(cortex_m::singleton!(: State<'static> = State::new()));

    let mut driver_config = UsbDriverConfig::default();
    driver_config.vbus_detection = true;

    let driver = UsbDriver::new_fs(usb, Irqs, dp, dm, ep_out_buffer, driver_config);

    let mut usb_config = UsbConfig::new(USB_VID, USB_PID);
    usb_config.manufacturer = Some("Raylar");
    usb_config.product = Some("Raylar USB CDC smoke test");
    usb_config.serial_number = Some("27_USB_CDC");
    usb_config.device_class = USB_CLASS_CDC;
    usb_config.device_sub_class = 0x02;
    usb_config.device_protocol = 0x00;
    usb_config.composite_with_iads = false;

    let mut builder = Builder::new(
        driver,
        usb_config,
        config_descriptor,
        bos_descriptor,
        msos_descriptor,
        control_buf,
    );

    let mut cdc = CdcAcmClass::new(&mut builder, cdc_state, CDC_MAX_PACKET_SIZE);
    let usb = builder.build();

    spawner.spawn(unwrap!(usb_task(usb)));

    loop {
        info!("Waiting for USB CDC host connection");
        cdc.wait_connection().await;
        info!("USB CDC connected");

        loop {
            activity_led.set_high();
            match cdc.write_packet(b"hello world\n").await {
                Ok(()) => info!("USB CDC sent hello world"),
                Err(e) => {
                    warn!("USB CDC write failed: {:?}", e);
                    activity_led.set_low();
                    break;
                }
            }
            activity_led.set_low();
            Timer::after_secs(1).await;
        }
    }
}

fn log_vbus(vbus: Peri<'static, PA9>) {
    let vbus = Input::new(vbus, Pull::None);
    info!("USB_VBUS divider input high={}", vbus.is_high());
}

#[embassy_executor::task]
async fn usb_task(mut usb: UsbDevice<'static, UsbCdcDriver>) -> ! {
    usb.run().await
}
