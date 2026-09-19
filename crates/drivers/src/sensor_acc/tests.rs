use super::*;
use embedded_hal::i2c::{ErrorKind, ErrorType, Operation};
use std::collections::VecDeque;
use std::vec;
use std::vec::Vec;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum MockError {
    Failure,
}

impl embedded_hal::i2c::Error for MockError {
    fn kind(&self) -> ErrorKind {
        ErrorKind::Other
    }
}

#[derive(Debug)]
enum Expected {
    Write {
        address: u8,
        bytes: Vec<u8>,
    },
    WriteRead {
        address: u8,
        write: Vec<u8>,
        response: Result<Vec<u8>, MockError>,
    },
}

#[derive(Debug)]
struct MockI2c {
    expected: VecDeque<Expected>,
}

impl MockI2c {
    fn new(expected: Vec<Expected>) -> Self {
        Self {
            expected: expected.into(),
        }
    }

    fn done(self) {
        assert!(
            self.expected.is_empty(),
            "unused transactions: {:?}",
            self.expected
        );
    }
}

impl ErrorType for MockI2c {
    type Error = MockError;
}

impl I2c for MockI2c {
    fn read(&mut self, _address: u8, _read: &mut [u8]) -> Result<(), Self::Error> {
        panic!("unexpected read")
    }

    fn write(&mut self, address: u8, write: &[u8]) -> Result<(), Self::Error> {
        match self.expected.pop_front().expect("unexpected write") {
            Expected::Write {
                address: expected_address,
                bytes,
            } => {
                assert_eq!(address, expected_address);
                assert_eq!(write, bytes);
                Ok(())
            }
            other => panic!("expected {other:?}, got write"),
        }
    }

    fn write_read(
        &mut self,
        address: u8,
        write: &[u8],
        read: &mut [u8],
    ) -> Result<(), Self::Error> {
        match self.expected.pop_front().expect("unexpected write_read") {
            Expected::WriteRead {
                address: expected_address,
                write: expected_write,
                response,
            } => {
                assert_eq!(address, expected_address);
                assert_eq!(write, expected_write);
                let response = response?;
                assert_eq!(read.len(), response.len());
                read.copy_from_slice(&response);
                Ok(())
            }
            other => panic!("expected {other:?}, got write_read"),
        }
    }

    fn transaction(
        &mut self,
        _address: u8,
        _operations: &mut [Operation<'_>],
    ) -> Result<(), Self::Error> {
        panic!("unexpected transaction")
    }
}

fn write(bytes: &[u8]) -> Expected {
    Expected::Write {
        address: DEVICE_ADDRESS,
        bytes: bytes.into(),
    }
}

fn read(register: u8, response: &[u8]) -> Expected {
    Expected::WriteRead {
        address: DEVICE_ADDRESS,
        write: [register].into(),
        response: Ok(response.into()),
    }
}

fn initialized(mut additional: Vec<Expected>) -> MockI2c {
    let mut expected = vec![
        read(REG_WHO_AM_I, &[DEVICE_ID]),
        write(&[REG_CTRL1, CTRL1_BDU_XYZ]),
        read(REG_CTRL4, &[0]),
        write(&[REG_CTRL4, CTRL4_IF_ADD_INC]),
    ];
    expected.append(&mut additional);
    MockI2c::new(expected)
}

fn turn_on_transactions(scale: FullScale, odr: OutputDataRate) -> Vec<Expected> {
    vec![
        read(REG_CTRL4, &[0]),
        write(&[
            REG_CTRL4,
            (scale.register_value() << CTRL4_FS_SHIFT) | CTRL4_IF_ADD_INC,
        ]),
        write(&[
            REG_CTRL1,
            (odr.register_value() << CTRL1_ODR_SHIFT) | CTRL1_BDU_XYZ,
        ]),
    ]
}

#[test]
fn defaults_to_two_g_and_ten_hz_and_starts_off() {
    let driver = Lis2hh12::new(initialized(vec![])).unwrap();
    assert_eq!(driver.config(), AccelerometerConfig::default());
    assert!(!driver.is_on());
    driver.into_inner().done();
}

#[test]
fn identity_mismatch_is_distinct_from_bus_error() {
    let wrong = MockI2c::new(vec![read(REG_WHO_AM_I, &[0x00])]);
    let result = Lis2hh12::new(wrong);
    assert!(matches!(
        result,
        Err(Error::DeviceIdMismatch { observed: 0x00 })
    ));

    let bus_error = MockI2c::new(vec![Expected::WriteRead {
        address: DEVICE_ADDRESS,
        write: vec![REG_WHO_AM_I],
        response: Err(MockError::Failure),
    }]);
    assert!(matches!(
        Lis2hh12::new(bus_error),
        Err(Error::Bus(MockError::Failure))
    ));
}

#[test]
fn all_odr_encodings_are_applied() {
    let rates = [
        OutputDataRate::Hz10,
        OutputDataRate::Hz50,
        OutputDataRate::Hz100,
        OutputDataRate::Hz200,
        OutputDataRate::Hz400,
        OutputDataRate::Hz800,
    ];

    for odr in rates {
        let bus = initialized(turn_on_transactions(FullScale::G2, odr));
        let mut driver = Lis2hh12::new(bus).unwrap();
        driver
            .configure(AccelerometerConfig {
                full_scale: FullScale::G2,
                odr,
            })
            .unwrap();
        driver.turn_on().unwrap();
        driver.into_inner().done();
    }
}

#[test]
fn all_full_scale_encodings_are_applied() {
    for full_scale in [FullScale::G2, FullScale::G4, FullScale::G8] {
        let bus = initialized(turn_on_transactions(full_scale, OutputDataRate::Hz10));
        let mut driver = Lis2hh12::new(bus).unwrap();
        driver
            .configure(AccelerometerConfig {
                full_scale,
                odr: OutputDataRate::Hz10,
            })
            .unwrap();
        driver.turn_on().unwrap();
        driver.into_inner().done();
    }
}

#[test]
fn configure_applies_changes_immediately_while_enabled() {
    let mut transactions = turn_on_transactions(FullScale::G2, OutputDataRate::Hz10);
    transactions.extend([
        read(REG_CTRL4, &[CTRL4_IF_ADD_INC]),
        write(&[
            REG_CTRL4,
            (FullScale::G8.register_value() << CTRL4_FS_SHIFT) | CTRL4_IF_ADD_INC,
        ]),
        write(&[
            REG_CTRL1,
            (OutputDataRate::Hz800.register_value() << CTRL1_ODR_SHIFT) | CTRL1_BDU_XYZ,
        ]),
    ]);
    let mut driver = Lis2hh12::new(initialized(transactions)).unwrap();
    driver.turn_on().unwrap();

    let config = AccelerometerConfig {
        full_scale: FullScale::G8,
        odr: OutputDataRate::Hz800,
    };
    driver.configure(config).unwrap();
    assert_eq!(driver.config(), config);
    assert!(driver.is_on());
    driver.into_inner().done();
}

#[test]
fn turn_off_powers_down_and_blocks_reads() {
    let mut transactions = turn_on_transactions(FullScale::G2, OutputDataRate::Hz10);
    transactions.push(write(&[REG_CTRL1, CTRL1_BDU_XYZ]));
    let mut driver = Lis2hh12::new(initialized(transactions)).unwrap();
    driver.turn_on().unwrap();
    driver.turn_off().unwrap();
    assert!(!driver.is_on());
    assert_eq!(driver.read_raw_acceleration(), Err(Error::NotEnabled));
    assert_eq!(driver.read_die_temperature(), Err(Error::NotEnabled));
    driver.into_inner().done();
}

#[test]
fn acceleration_is_decoded_and_converted_without_floating_point() {
    let mut transactions = turn_on_transactions(FullScale::G2, OutputDataRate::Hz10);
    transactions.push(read(
        REG_OUT_X_L | AUTO_INCREMENT,
        &[0x00, 0x40, 0x00, 0xC0, 0x01, 0x00],
    ));
    let mut driver = Lis2hh12::new(initialized(transactions)).unwrap();
    driver.turn_on().unwrap();

    assert_eq!(
        driver.read_acceleration().unwrap(),
        Acceleration {
            x_mg: 999,
            y_mg: -999,
            z_mg: 0,
        }
    );
    driver.into_inner().done();
}

#[test]
fn die_temperature_uses_signed_eleven_bit_value() {
    let mut transactions = turn_on_transactions(FullScale::G2, OutputDataRate::Hz10);
    transactions.push(read(REG_TEMP_L | AUTO_INCREMENT, &[0x00, 0xFF]));
    let mut driver = Lis2hh12::new(initialized(transactions)).unwrap();
    driver.turn_on().unwrap();

    assert_eq!(
        driver.read_die_temperature().unwrap(),
        DieTemperature {
            raw: -8,
            milli_celsius: 24_000,
        }
    );
    driver.into_inner().done();
}

#[test]
fn data_ready_reads_status_register() {
    let mut transactions = turn_on_transactions(FullScale::G2, OutputDataRate::Hz10);
    transactions.push(read(REG_STATUS, &[STATUS_XYZ_NEW_DATA]));
    let mut driver = Lis2hh12::new(initialized(transactions)).unwrap();
    driver.turn_on().unwrap();
    assert_eq!(driver.is_data_ready(), Ok(true));
    driver.into_inner().done();
}
