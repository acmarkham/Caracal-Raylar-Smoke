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

fn cfg_a(enabled: bool, odr: OutputDataRate) -> u8 {
    CFG_A_TEMP_COMPENSATION
        | (odr.register_value() << CFG_A_ODR_SHIFT)
        | if enabled {
            CFG_A_MODE_CONTINUOUS
        } else {
            CFG_A_MODE_POWER_DOWN
        }
}

fn initialized(mut additional: Vec<Expected>) -> MockI2c {
    let mut expected = vec![
        read(REG_WHO_AM_I, &[DEVICE_ID]),
        write(&[REG_CFG_A, cfg_a(false, OutputDataRate::Hz10)]),
        write(&[REG_CFG_B, CFG_B_OFFSET_CANCELLATION_EVERY_ODR]),
        write(&[REG_CFG_C, CFG_C_BLOCK_DATA_UPDATE]),
    ];
    expected.append(&mut additional);
    MockI2c::new(expected)
}

fn turn_on_transactions(odr: OutputDataRate) -> Vec<Expected> {
    vec![
        write(&[REG_CFG_B, CFG_B_OFFSET_CANCELLATION_EVERY_ODR]),
        write(&[REG_CFG_C, CFG_C_BLOCK_DATA_UPDATE]),
        write(&[REG_CFG_A, cfg_a(true, odr)]),
    ]
}

#[test]
fn defaults_to_ten_hz_high_resolution_and_starts_off() {
    let driver = Lis2mdl::new(initialized(vec![])).unwrap();
    assert_eq!(driver.config(), MagnetometerConfig::default());
    assert!(!driver.is_on());
    driver.into_inner().done();
}

#[test]
fn identity_mismatch_is_distinct_from_bus_error() {
    let wrong = MockI2c::new(vec![read(REG_WHO_AM_I, &[0x00])]);
    assert!(matches!(
        Lis2mdl::new(wrong),
        Err(Error::DeviceIdMismatch { observed: 0x00 })
    ));

    let bus_error = MockI2c::new(vec![Expected::WriteRead {
        address: DEVICE_ADDRESS,
        write: vec![REG_WHO_AM_I],
        response: Err(MockError::Failure),
    }]);
    assert!(matches!(
        Lis2mdl::new(bus_error),
        Err(Error::Bus(MockError::Failure))
    ));
}

#[test]
fn all_odr_encodings_are_applied_with_fixed_defaults() {
    for odr in [
        OutputDataRate::Hz10,
        OutputDataRate::Hz20,
        OutputDataRate::Hz50,
        OutputDataRate::Hz100,
    ] {
        let mut driver = Lis2mdl::new(initialized(turn_on_transactions(odr))).unwrap();
        driver.configure(MagnetometerConfig { odr }).unwrap();
        driver.turn_on().unwrap();
        assert!(driver.is_on());
        driver.into_inner().done();
    }
}

#[test]
fn configure_applies_new_odr_immediately_while_enabled() {
    let mut transactions = turn_on_transactions(OutputDataRate::Hz10);
    transactions.push(write(&[REG_CFG_A, cfg_a(true, OutputDataRate::Hz100)]));
    let mut driver = Lis2mdl::new(initialized(transactions)).unwrap();
    driver.turn_on().unwrap();
    let config = MagnetometerConfig {
        odr: OutputDataRate::Hz100,
    };
    driver.configure(config).unwrap();
    assert_eq!(driver.config(), config);
    assert!(driver.is_on());
    driver.into_inner().done();
}

#[test]
fn turn_off_powers_down_and_blocks_reads() {
    let mut transactions = turn_on_transactions(OutputDataRate::Hz10);
    transactions.push(write(&[REG_CFG_A, cfg_a(false, OutputDataRate::Hz10)]));
    let mut driver = Lis2mdl::new(initialized(transactions)).unwrap();
    driver.turn_on().unwrap();
    driver.turn_off().unwrap();
    assert!(!driver.is_on());
    assert_eq!(driver.read_raw_magnetic_field(), Err(Error::NotEnabled));
    assert_eq!(driver.read_die_temperature(), Err(Error::NotEnabled));
    driver.into_inner().done();
}

#[test]
fn magnetic_field_is_decoded_and_converted_without_floating_point() {
    let mut transactions = turn_on_transactions(OutputDataRate::Hz10);
    transactions.push(read(
        REG_OUT_X_L | AUTO_INCREMENT,
        &[0x64, 0x00, 0x9C, 0xFF, 0x01, 0x00],
    ));
    let mut driver = Lis2mdl::new(initialized(transactions)).unwrap();
    driver.turn_on().unwrap();

    assert_eq!(
        driver.read_magnetic_field().unwrap(),
        MagneticField {
            x_nanotesla: 15_000,
            y_nanotesla: -15_000,
            z_nanotesla: 150,
        }
    );
    driver.into_inner().done();
}

#[test]
fn die_temperature_sign_extends_twelve_bit_value() {
    let mut transactions = turn_on_transactions(OutputDataRate::Hz10);
    transactions.push(read(REG_TEMP_OUT_L | AUTO_INCREMENT, &[0xF8, 0x0F]));
    let mut driver = Lis2mdl::new(initialized(transactions)).unwrap();
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
    let mut transactions = turn_on_transactions(OutputDataRate::Hz10);
    transactions.push(read(REG_STATUS, &[STATUS_XYZ_NEW_DATA]));
    let mut driver = Lis2mdl::new(initialized(transactions)).unwrap();
    driver.turn_on().unwrap();
    assert_eq!(driver.is_data_ready(), Ok(true));
    driver.into_inner().done();
}
