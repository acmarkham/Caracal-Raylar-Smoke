#![no_std]

use embassy_stm32::exti::{self, ExtiInput};
use embassy_stm32::gpio::{Input, Level, Output, Pull, Speed};
use embassy_stm32::mode::Async;
use embassy_stm32::Peri;
use embassy_stm32::Peripherals;
// ADC voltage sense imports
use embassy_stm32::peripherals::{ADC1, ADC4, PA0, PA1, PB1};
// buzzer imports
use embassy_stm32::peripherals::{PA5, TIM8};
// i2c imports
use embassy_stm32::peripherals::{I2C1, I2C5, PB6, PB7, PD0, PD1};
// gps imports
use embassy_stm32::peripherals::{PA2, PA3, USART2};
// pdm microphone imports
use embassy_stm32::peripherals::{PB8, PD3};
// microSD imports
use embassy_stm32::peripherals::{PC10, PC11, PC12, PC8, PC9, PD2, SDMMC1};
// Ebyte E80 LR1121 RF module imports
use embassy_stm32::peripherals::{PE13, PE14, PE15, SPI1};
use embassy_stm32::{bind_interrupts, interrupt, sdmmc, usart};

// Full STM32U595VJT6Q pin map for Raylar v1.00, extracted from the KiCad U1
// footprint/net assignments. This crate currently models only the subset needed
// by the existing smoke tests and board bring-up code.
//
// Implemented in this crate:
// - PE2  = SW_USER
// - PE3  = GPS_RESET
// - PA0  = V_ADC_DC
// - PA1  = V_ADC_BATT
// - PA2  = GPS_RX_STM_TX
// - PA3  = GPS_TX_STM_RX
// - PA5  = PWM_BUZ
// - PB4  = SYS_GPS_GREEN
// - PB1  = V_ADC_SOLAR
// - PB6  = QWIIC_SCL
// - PB7  = QWIIC_SDA
// - PB8  = MIC_CCLK0
// - PB9  = GPS_PPS
// - PB15 = SYS_MAIN_RED
// - PC8  = SDIO_D0
// - PC9  = SDIO_D1
// - PC10 = SDIO_D2
// - PC11 = SDIO_D3
// - PC12 = SDIO_CLK
// - PC13 = GPS_EN
// - PD0  = SENS_I2C_SDA
// - PD1  = SENS_I2C_SCL
// - PD2  = SDIO_CMD
// - PD3  = MIC_SD0
// - PD4  = SD_SW
// - PD5  = SYS_SD_BLUE
// - PD7  = SYS_GPS_RED
// - PD10 = SYS_MAIN_GREEN
// - PE0  = SD_PWR
// - PE8  = RF_CS
// - PE10 = RF_BUSY
// - PE11 = RF_NRST
// - PE12 = RF_IRQ
// - PE13 = RF_SCK
// - PE14 = RF_MISO
// - PE15 = RF_MOSI
//
// Present on the MCU but not yet modeled in this crate:
// - PA4  = MBUS_AN
// - PA6  = EXT_OPA_VINP
// - PA7  = EXT_OPA_VINM
// - PA8  = RCC_MCO
// - PA9  = USB_VBUS
// - PA10 = unconnected
// - PA11 = USB_D_N
// - PA12 = USB_D_P
// - PA13 = TRACE_SWDIO
// - PA14 = TRACE_SWCLK
// - PA15 = unconnected
// - PB0  = EXT_OPA_VOUT
// - PB2  = MBUS_PWM
// - PB3  = TRACE_SWO
// - PB5  = MBUS_INT
// - PB10 = EXT_I2C_SCL
// - PB11 = MBUS_RX_STM_RX
// - PB13 = MBUS_SCK
// - PB14 = MBUS_MISO
// - PC0  = MBUS_SCL
// - PC1  = MBUS_SDA
// - PC2  = MIC_CCLK1
// - PC3  = MBUS_MOSI
// - PC6  = EXT_1
// - PC7  = EXT_2
// - PD6  = MIC_SD1
// - PD8  = MBUS_TX_STM_TX
// - PD9  = EXT_AN_IN
// - PD11 = EXT_3
// - PD12 = unconnected
// - PD13 = EXT_I2C_SDA
// - PD14 = unconnected
// - PD15 = unconnected
// - PE4  = MIC_SD3
// - PE5  = MBUS_CS
// - PE6  = MBUS_RST
// - PE7  = MIC_SD2
// - PE9  = CHG_INT
// - PH3  = BOOT0
//
// Power, clocks, and other non-GPIO pins on U1:
// - VBAT
// - VSS, VSSA, VSSSMPS
// - VDD, VDDA, VDDUSB, VDD11, VDDSMPS, VLXSMPS
// - VREF+
// - PH0 = HSE_OSC_IN
// - PH1 = HSE_OSC_OUT
// - PC14 = 32K_OSC_IN
// - PC15 = 32K_OSC_OUT
// - NRST

bind_interrupts!(pub struct Irqs {
    EXTI2 => exti::InterruptHandler<interrupt::typelevel::EXTI2>;
    EXTI9 => exti::InterruptHandler<interrupt::typelevel::EXTI9>;
    EXTI12 => exti::InterruptHandler<interrupt::typelevel::EXTI12>;
    SDMMC1 => sdmmc::InterruptHandler<SDMMC1>;
    USART2 => usart::BufferedInterruptHandler<USART2>;
});

pub struct Board<'d> {
    pub leds: Leds<'d>,
    pub buttons: Buttons<'d>,
    pub adc_voltages: AdcVoltages<'d>,
    pub buzzer: Buzzer<'d>,
    pub sens_i2c: SensI2C<'d>,
    pub qwiic_i2c: QwiicI2C<'d>,
    pub gps: Gps<'d>,
    pub pdm_mic1: PdmMic1<'d>,
    pub sd: SdCard<'d>,
    pub ebyte_rf: EbyteRf<'d>,
}

pub struct Leds<'d> {
    pub sys_gps_green: Output<'d>,
    pub sys_gps_red: Output<'d>,
    pub sys_main_red: Output<'d>,
    pub sys_main_green: Output<'d>,
    pub sys_sd_blue: Output<'d>,
}

pub struct Buttons<'d> {
    pub user: ExtiInput<'d, Async>,
}

// ADC voltage sense pins on ADC1.
pub struct AdcVoltages<'d> {
    pub adc: Peri<'d, ADC1>,
    pub adc4: Peri<'d, ADC4>,
    pub v_dc: Peri<'d, PA0>,
    pub v_batt: Peri<'d, PA1>,
    pub v_solar: Peri<'d, PB1>,
}

// Buzzer on PA5, TIM8_CH1N (can also be exposed as DAC1_OUT2)
pub struct Buzzer<'d> {
    pub tim: Peri<'d, TIM8>,
    pub pin: Peri<'d, PA5>,
}

// SENS_I2C (accelerometer, magnetometer and BMC charge controller) on PD0 (SDA) and PD1 (SCL) on I2C5.
pub struct SensI2C<'d> {
    pub i2c: Peri<'d, I2C5>,
    pub sda: Peri<'d, PD0>,
    pub scl: Peri<'d, PD1>,
}

// QWIIC connector on PB7 (SDA) and PB6 (SCL) using I2C1.
pub struct QwiicI2C<'d> {
    pub i2c: Peri<'d, I2C1>,
    pub sda: Peri<'d, PB7>,
    pub scl: Peri<'d, PB6>,
}

// GPS on USART2: STM TX PA2 -> GPS RX, GPS TX -> STM RX PA3, PPS on PB9/EXTI9.
// Control pins from schematic/PCB: PE3 = GPS_RESET, PC13 = GPS_EN.
pub struct Gps<'d> {
    pub usart: Peri<'d, USART2>,
    pub tx: Peri<'d, PA2>,
    pub rx: Peri<'d, PA3>,
    pub pps: ExtiInput<'d, Async>,
    pub rst: Output<'d>,
    pub en: Output<'d>,
}

// PDM_MIC1 on MDF1: MIC_CCLK0 PB8 -> MDF1_CCK0, schematic MDF_SDIO1 PD3 -> MDF1_SDI0.
pub struct PdmMic1<'d> {
    pub cck0: Peri<'d, PB8>,
    pub sdio: Peri<'d, PD3>,
}

// microSD on SDMMC1 4-bit default pins. SD_SW is low when a card is inserted.
pub struct SdCard<'d> {
    pub sdmmc: Peri<'d, SDMMC1>,
    pub clk: Peri<'d, PC12>,
    pub cmd: Peri<'d, PD2>,
    pub d0: Peri<'d, PC8>,
    pub d1: Peri<'d, PC9>,
    pub d2: Peri<'d, PC10>,
    pub d3: Peri<'d, PC11>,
    pub switch: Input<'d>,
    pub power: Output<'d>,
}

// Ebyte E80 LR1121 RF module on SPI1, with manual chip select and DIO9 IRQ.
pub struct EbyteRf<'d> {
    pub spi: Peri<'d, SPI1>,
    pub sck: Peri<'d, PE13>,
    pub miso: Peri<'d, PE14>,
    pub mosi: Peri<'d, PE15>,
    pub cs: Output<'d>,
    pub busy: Input<'d>,
    pub nrst: Output<'d>,
    pub irq: ExtiInput<'d, Async>,
}

impl Board<'static> {
    pub fn new(p: Peripherals) -> Self {
        let Peripherals {
            // leds
            PB4,
            PD7,
            PB15,
            PD10,
            PD5,
            // user button
            PE2,
            EXTI2,
            // adc voltage sense
            ADC1,
            ADC4,
            PA0,
            PA1,
            PB1,
            // buzzer
            PA5,
            TIM8,
            // sens_i2c
            PD0,
            PD1,
            I2C5,
            // qwiic_i2c
            PB6,
            PB7,
            I2C1,
            // gps
            PC13,
            PE3,
            PB9,
            EXTI9,
            PA2,
            PA3,
            USART2,
            // pdm_mic1
            PB8,
            PD3,
            // microSD
            SDMMC1,
            PC12,
            PD2,
            PC8,
            PC9,
            PC10,
            PC11,
            PD4,
            PE0,
            // Ebyte E80 LR1121 RF module
            SPI1,
            PE8,
            PE10,
            PE11,
            PE12,
            EXTI12,
            PE13,
            PE14,
            PE15,
            ..
        } = p;

        Self {
            leds: Leds {
                sys_gps_green: Output::new(PB4, Level::Low, Speed::Medium),
                sys_gps_red: Output::new(PD7, Level::Low, Speed::Medium),
                sys_main_red: Output::new(PB15, Level::Low, Speed::Medium),
                sys_main_green: Output::new(PD10, Level::Low, Speed::Medium),
                sys_sd_blue: Output::new(PD5, Level::Low, Speed::Medium),
            },
            buttons: Buttons {
                user: ExtiInput::new(PE2, EXTI2, Pull::Up, Irqs),
            },
            adc_voltages: AdcVoltages {
                adc: ADC1,
                adc4: ADC4,
                v_dc: PA0,
                v_batt: PA1,
                v_solar: PB1,
            },
            buzzer: Buzzer {
                tim: TIM8,
                pin: PA5,
            },
            sens_i2c: SensI2C {
                i2c: I2C5,
                sda: PD0,
                scl: PD1,
            },
            qwiic_i2c: QwiicI2C {
                i2c: I2C1,
                sda: PB7,
                scl: PB6,
            },
            gps: Gps {
                usart: USART2,
                tx: PA2,
                rx: PA3,
                pps: ExtiInput::new(PB9, EXTI9, Pull::None, Irqs),
                rst: Output::new(PE3, Level::Low, Speed::Medium),
                en: Output::new(PC13, Level::Low, Speed::Medium),
            },
            pdm_mic1: PdmMic1 {
                cck0: PB8,
                sdio: PD3,
            },
            sd: SdCard {
                sdmmc: SDMMC1,
                clk: PC12,
                cmd: PD2,
                d0: PC8,
                d1: PC9,
                d2: PC10,
                d3: PC11,
                switch: Input::new(PD4, Pull::Up),
                power: Output::new(PE0, Level::High, Speed::Medium),
            },
            ebyte_rf: EbyteRf {
                spi: SPI1,
                sck: PE13,
                miso: PE14,
                mosi: PE15,
                cs: Output::new(PE8, Level::High, Speed::VeryHigh),
                busy: Input::new(PE10, Pull::None),
                nrst: Output::new(PE11, Level::High, Speed::Medium),
                irq: ExtiInput::new(PE12, EXTI12, Pull::None, Irqs),
            },
        }
    }
}
