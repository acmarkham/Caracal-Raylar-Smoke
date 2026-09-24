//! Minimal TMF8829 protocol used by the LightRanger 14 smoke test.
//!
//! This follows the MikroE LightRanger 14 example's firmware-download and
//! default 8x8 configuration sequence. It is intentionally local to the smoke
//! test until the API has been exercised on hardware.

use defmt::{info, Format};
use embassy_stm32::gpio::Output;
use embassy_stm32::i2c::mode::Master;
use embassy_stm32::i2c::{Error as I2cError, I2c};
use embassy_stm32::mode::Blocking;
use embassy_time::Timer;

use crate::firmware;

const ADDRESS: u8 = 0x41;
const MAX_TRANSFER: usize = 128;
const TIMEOUT_MS: usize = 1_000;

const REG_APP_ID: u8 = 0x00;
const REG_CMD_STATUS: u8 = 0x08;
const REG_CID_RID: u8 = 0x09;
const REG_SERIAL_NUMBER: u8 = 0x1c;
const REG_PERIOD_LSB: u8 = 0x22;
const REG_INTERRUPT_STATUS: u8 = 0xe1;
const REG_INTERRUPT_ENABLE: u8 = 0xe2;
const REG_CHIP_ID: u8 = 0xe3;
const REG_FIFO_STATUS: u8 = 0xfa;
const REG_FIFO: u8 = 0xff;

const APP_ID_RAM: u8 = 0x01;
const APP_ID_BOOTLOADER: u8 = 0x80;
const EXPECTED_CHIP_ID: u8 = 0x9e;

const CMD_APP_MEASURE: u8 = 0x10;
const CMD_APP_WRITE_PAGE: u8 = 0x15;
const CMD_APP_LOAD_CONFIG_PAGE: u8 = 0x16;
const CMD_APP_LOAD_8X8: u8 = 0x40;
const CMD_BOOTLOADER_START_RAM: u8 = 0x16;
const CMD_BOOTLOADER_SPI_OFF: u8 = 0x20;
const CMD_BOOTLOADER_WRITE_FIFO: u8 = 0x45;
const CMD_STATUS_OK: u8 = 0x00;
const CMD_STATUS_ACCEPTED: u8 = 0x01;
const CID_CONFIG_PAGE: u8 = 0x16;

const INTERRUPT_RESULTS: u8 = 0x01;
const MEASUREMENT_PERIOD_MS: u16 = 500;
const IMAGE_START: u32 = 0x0001_0000;

const PREHEADER_SIZE: usize = 5;
const HEADER_SIZE: usize = 16;
const FOOTER_SIZE: usize = 12;
const FRAME_HEADER_LENGTH_BIAS: usize = HEADER_SIZE - 4;
const FRAME_EOF_MARKER: u16 = 0xe0f7;
const FRAME_ID_RESULT: u8 = 0x10;
const FRAME_ID_MASK: u8 = 0xf0;
const FRAME_STATUS_VALID: u8 = 0x01;

pub const MAP_WIDTH: usize = 8;
pub const MAP_HEIGHT: usize = 8;
const PIXEL_SIZE: usize = 3;
const PAYLOAD_SIZE: usize = MAP_WIDTH * MAP_HEIGHT * PIXEL_SIZE;

pub const CONFIDENCE_THRESHOLD: u32 = 6;
const CONFIDENCE_BREAKPOINT: u8 = 40;
const CONFIDENCE_GROWTH_SCALED: u64 = 1_053_676;
const CONFIDENCE_SCALE: u64 = 1_000_000;

type BlockingI2c = I2c<'static, Blocking, Master>;

#[derive(Clone, Copy, Format)]
pub enum Error {
    I2c(I2cError),
    Timeout {
        register: u8,
        expected: u8,
        actual: u8,
    },
    UnexpectedChipId(u8),
    InvalidPayloadLength(u16),
    InvalidEndMarker(u16),
}

impl From<I2cError> for Error {
    fn from(value: I2cError) -> Self {
        Self::I2c(value)
    }
}

pub struct DeviceInfo {
    pub app_version: [u8; 4],
    pub chip_version: [u8; 2],
    pub serial_number: u32,
}

pub struct Pixel {
    pub distance_quarter_mm: u16,
    pub confidence_code: u8,
}

pub struct Frame {
    pub systick: u32,
    pub frame_id: u8,
    pub frame_number: u32,
    pub temperature: [u8; 3],
    pub status: u8,
    payload_len: usize,
    payload: [u8; PAYLOAD_SIZE],
}

impl Frame {
    fn empty() -> Self {
        Self {
            systick: 0,
            frame_id: 0,
            frame_number: 0,
            temperature: [0; 3],
            status: 0,
            payload_len: 0,
            payload: [0; PAYLOAD_SIZE],
        }
    }

    pub fn is_valid_result(&self) -> bool {
        self.status & FRAME_STATUS_VALID != 0
            && self.frame_id & FRAME_ID_MASK == FRAME_ID_RESULT
            && self.payload_len == PAYLOAD_SIZE
    }

    pub fn pixel(&self, index: usize) -> Pixel {
        let offset = index * PIXEL_SIZE;
        Pixel {
            distance_quarter_mm: u16::from_le_bytes([
                self.payload[offset],
                self.payload[offset + 1],
            ]),
            confidence_code: self.payload[offset + 2],
        }
    }
}

pub struct Tmf8829<'a> {
    i2c: &'a mut BlockingI2c,
}

impl<'a> Tmf8829<'a> {
    pub fn new(i2c: &'a mut BlockingI2c) -> Self {
        Self { i2c }
    }

    pub async fn initialize(&mut self, enable: &mut Output<'static>) -> Result<DeviceInfo, Error> {
        enable.set_low();
        Timer::after_millis(100).await;
        enable.set_high();
        Timer::after_millis(100).await;

        self.download_firmware().await?;
        let info = self.read_device_info()?;

        self.write_command(CMD_APP_LOAD_CONFIG_PAGE, CMD_STATUS_OK)
            .await?;
        self.check_register(REG_CID_RID, CID_CONFIG_PAGE).await?;
        self.write_registers(REG_PERIOD_LSB, &MEASUREMENT_PERIOD_MS.to_le_bytes())?;
        self.write_command(CMD_APP_WRITE_PAGE, CMD_STATUS_OK)
            .await?;
        self.write_register(REG_INTERRUPT_ENABLE, INTERRUPT_RESULTS)?;
        self.write_command(CMD_APP_LOAD_8X8, CMD_STATUS_OK).await?;
        self.write_command(CMD_APP_MEASURE, CMD_STATUS_ACCEPTED)
            .await?;
        self.clear_interrupts()?;

        Ok(info)
    }

    async fn download_firmware(&mut self) -> Result<(), Error> {
        self.check_register(REG_APP_ID, APP_ID_BOOTLOADER).await?;
        self.write_command(CMD_BOOTLOADER_SPI_OFF, CMD_STATUS_OK)
            .await?;

        let image_words = firmware::IMAGE.len().div_ceil(4) as u16;
        let address = IMAGE_START.to_le_bytes();
        let words = image_words.to_le_bytes();
        let fifo_setup = [
            CMD_BOOTLOADER_WRITE_FIFO,
            6,
            address[0],
            address[1],
            address[2],
            address[3],
            words[0],
            words[1],
        ];
        self.write_registers(REG_CMD_STATUS, &fifo_setup)?;
        self.check_register(REG_CMD_STATUS, CMD_STATUS_OK).await?;

        for (chunk_index, chunk) in firmware::IMAGE.chunks(MAX_TRANSFER).enumerate() {
            self.write_registers(REG_FIFO, chunk)?;
            if chunk_index % 16 == 15
                || chunk_index + 1 == firmware::IMAGE.len().div_ceil(MAX_TRANSFER)
            {
                info!(
                    "TMF8829 firmware: {}/{} bytes",
                    core::cmp::min((chunk_index + 1) * MAX_TRANSFER, firmware::IMAGE.len()),
                    firmware::IMAGE.len()
                );
            }
        }

        self.write_command(CMD_BOOTLOADER_START_RAM, CMD_STATUS_OK)
            .await?;
        self.check_register(REG_APP_ID, APP_ID_RAM).await
    }

    fn read_device_info(&mut self) -> Result<DeviceInfo, Error> {
        let mut app_version = [0; 4];
        self.read_registers(REG_APP_ID, &mut app_version)?;

        let mut serial_bytes = [0; 4];
        self.read_registers(REG_SERIAL_NUMBER, &mut serial_bytes)?;

        let mut chip_version = [0; 2];
        self.read_registers(REG_CHIP_ID, &mut chip_version)?;
        if chip_version[0] != EXPECTED_CHIP_ID {
            return Err(Error::UnexpectedChipId(chip_version[0]));
        }

        Ok(DeviceInfo {
            app_version,
            chip_version,
            serial_number: u32::from_le_bytes(serial_bytes),
        })
    }

    pub fn clear_interrupts(&mut self) -> Result<(), Error> {
        let status = self.read_register(REG_INTERRUPT_STATUS)?;
        self.write_register(REG_INTERRUPT_STATUS, status)
    }

    pub fn read_frame(&mut self) -> Result<Frame, Error> {
        let mut frame = Frame::empty();
        let mut header = [0; PREHEADER_SIZE + HEADER_SIZE];
        self.read_registers(REG_FIFO_STATUS, &mut header)?;

        frame.systick = u32::from_le_bytes([header[1], header[2], header[3], header[4]]);
        frame.frame_id = header[5];
        let reported_length = u16::from_le_bytes([header[7], header[8]]);
        let overhead = (FOOTER_SIZE + FRAME_HEADER_LENGTH_BIAS) as u16;
        let Some(payload_len) = reported_length.checked_sub(overhead) else {
            return Err(Error::InvalidPayloadLength(reported_length));
        };
        if payload_len == 0 || usize::from(payload_len) > PAYLOAD_SIZE {
            return Err(Error::InvalidPayloadLength(payload_len));
        }
        frame.payload_len = usize::from(payload_len);
        frame.frame_number = u32::from_le_bytes([header[9], header[10], header[11], header[12]]);
        frame.temperature.copy_from_slice(&header[13..16]);

        for chunk in frame.payload[..frame.payload_len].chunks_mut(MAX_TRANSFER) {
            self.read_registers(REG_FIFO, chunk)?;
        }

        let mut footer = [0; FOOTER_SIZE];
        self.read_registers(REG_FIFO, &mut footer)?;
        frame.status = footer[8];
        let eof_marker = u16::from_le_bytes([footer[10], footer[11]]);
        if eof_marker != FRAME_EOF_MARKER {
            return Err(Error::InvalidEndMarker(eof_marker));
        }

        Ok(frame)
    }

    async fn write_command(&mut self, command: u8, expected: u8) -> Result<(), Error> {
        self.write_register(REG_CMD_STATUS, command)?;
        self.check_register(REG_CMD_STATUS, expected).await
    }

    async fn check_register(&mut self, register: u8, expected: u8) -> Result<(), Error> {
        let mut actual = 0;
        for _ in 0..=TIMEOUT_MS {
            actual = self.read_register(register)?;
            if actual == expected {
                return Ok(());
            }
            Timer::after_millis(1).await;
        }
        Err(Error::Timeout {
            register,
            expected,
            actual,
        })
    }

    fn write_register(&mut self, register: u8, value: u8) -> Result<(), Error> {
        self.write_registers(register, &[value])
    }

    fn write_registers(&mut self, register: u8, values: &[u8]) -> Result<(), Error> {
        let mut buffer = [0; MAX_TRANSFER + 1];
        buffer[0] = register;
        buffer[1..=values.len()].copy_from_slice(values);
        self.i2c
            .blocking_write(ADDRESS, &buffer[..=values.len()])
            .map_err(Error::from)
    }

    fn read_register(&mut self, register: u8) -> Result<u8, Error> {
        let mut value = [0];
        self.read_registers(register, &mut value)?;
        Ok(value[0])
    }

    fn read_registers(&mut self, register: u8, values: &mut [u8]) -> Result<(), Error> {
        self.i2c
            .blocking_write_read(ADDRESS, &[register], values)
            .map_err(Error::from)
    }
}

/// Convert the sensor's logarithmically encoded SNR byte to the confidence
/// value used by the MikroE example. Fixed-point arithmetic avoids pulling a
/// floating-point `pow` implementation into this no_std smoke-test binary.
pub fn confidence(code: u8) -> u32 {
    if code <= CONFIDENCE_BREAKPOINT {
        return u32::from(code);
    }

    let mut scaled = u64::from(CONFIDENCE_BREAKPOINT) * CONFIDENCE_SCALE;
    for _ in CONFIDENCE_BREAKPOINT..code {
        scaled = (scaled * CONFIDENCE_GROWTH_SCALED + CONFIDENCE_SCALE / 2) / CONFIDENCE_SCALE;
    }
    ((scaled + CONFIDENCE_SCALE / 2) / CONFIDENCE_SCALE) as u32
}
