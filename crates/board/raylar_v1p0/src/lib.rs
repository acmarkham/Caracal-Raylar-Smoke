#![no_std]

use embassy_stm32::exti::{self, ExtiInput};
use embassy_stm32::gpio::{Input, Level, Output, Pull, Speed};
use embassy_stm32::mode::Async;
use embassy_stm32::Peripherals;
use embassy_stm32::Peri;
// buzzer imports
use embassy_stm32::peripherals::{PA5, TIM8};
// i2c imports
use embassy_stm32::peripherals::{I2C5, PD0, PD1};
// gps imports
use embassy_stm32::peripherals::{PA2, PA3, USART2};
// pdm microphone imports
use embassy_stm32::peripherals::{PB8, PD3};
// microSD imports
use embassy_stm32::peripherals::{PC8, PC9, PC10, PC11, PC12, PD2, SDMMC1};
// Ebyte E80 LR1121 RF module imports
use embassy_stm32::peripherals::{PE13, PE14, PE15, SPI1};
use embassy_stm32::{bind_interrupts, interrupt, sdmmc, usart};

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
    pub buzzer: Buzzer<'d>,
    pub sens_i2c: SensI2C<'d>,
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

// GPS on USART2: STM TX PA2 -> GPS RX, GPS TX -> STM RX PA3, PPS on PB9/EXTI9.
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
            // buzzer
            PA5,
            TIM8,
            // sens_i2c 
            PD0,
            PD1,
            I2C5,
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
            buzzer: Buzzer {
                tim: TIM8,
                pin: PA5,
            },
            sens_i2c: SensI2C {
                i2c: I2C5,
                sda: PD0,
                scl: PD1,
            },
            gps: Gps {
                usart: USART2,
                tx: PA2,
                rx: PA3,
                pps: ExtiInput::new(PB9, EXTI9, Pull::None, Irqs),
                rst: Output::new(PC13, Level::Low, Speed::Medium),
                en: Output::new(PE3, Level::Low, Speed::Medium),
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
