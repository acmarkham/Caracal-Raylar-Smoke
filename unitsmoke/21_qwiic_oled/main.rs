// QWIIC OLED smoke test for an SH1107 monochrome display.
//
// Activity: scan the QWIIC I2C bus on PB6/PB7, report any responding devices,
// initialize an SH1107 display at 0x3c or 0x3d, then animate one horizontal
// pixel row across the display.

#![no_std]
#![no_main]

use defmt::{info, unwrap, warn};
use embassy_executor::Spawner;
use embassy_stm32::gpio::Output;
use embassy_stm32::i2c::mode::Master;
use embassy_stm32::i2c::{Config, I2c};
use embassy_stm32::mode::Blocking;
use embassy_stm32::rcc::*;
use embassy_stm32::time::{mhz, Hertz};
use embassy_time::Timer;
use raylar_board_v1p0::{Board, Leds, QwiicI2C};
use {defmt_rtt as _, panic_probe as _};

const OLED_ADDR_PRIMARY: u8 = 0x3c;
const OLED_ADDR_SECONDARY: u8 = 0x3d;
const DISPLAY_WIDTH: usize = 128;
const DISPLAY_HEIGHT: u8 = 128;
const PAGE_HEIGHT: u8 = 8;
const DISPLAY_PAGES: u8 = DISPLAY_HEIGHT / PAGE_HEIGHT;

type BlockingI2c = I2c<'static, Blocking, Master>;

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
    let Board {
        leds, qwiic_i2c, ..
    } = Board::new(p);
    let Leds {
        sys_main_red,
        sys_main_green,
        ..
    } = leds;

    info!("QWIIC SH1107 OLED smoke test started");

    spawner.spawn(unwrap!(heartbeat_task(sys_main_green)));
    spawner.spawn(unwrap!(oled_task(qwiic_i2c, sys_main_red)));

    core::future::pending().await
}

#[embassy_executor::task]
async fn heartbeat_task(mut led: Output<'static>) -> ! {
    loop {
        led.set_high();
        Timer::after_millis(100).await;
        led.set_low();
        Timer::after_millis(900).await;
    }
}

#[embassy_executor::task]
async fn oled_task(qwiic_i2c: QwiicI2C<'static>, mut led: Output<'static>) -> ! {
    let QwiicI2C { i2c, scl, sda } = qwiic_i2c;
    let mut config = Config::default();
    config.frequency = Hertz(100_000);

    let mut i2c = I2c::new_blocking(i2c, scl, sda, config);

    led.set_high();
    let oled_addr = scan_qwiic_bus(&mut i2c);
    led.set_low();

    let Some(oled_addr) = oled_addr else {
        warn!("No SH1107 OLED found at 0x3c or 0x3d; continuing to rescan");
        loop {
            Timer::after_secs(2).await;
            led.set_high();
            let found = scan_qwiic_bus(&mut i2c);
            led.set_low();
            if let Some(addr) = found {
                info!("SH1107 OLED appeared at 0x{:02x}; starting animation", addr);
                init_sh1107(&mut i2c, addr);
                animate_scanline(&mut i2c, addr, &mut led).await;
            }
        }
    };

    info!(
        "SH1107 OLED found at 0x{:02x}; starting animation",
        oled_addr
    );
    init_sh1107(&mut i2c, oled_addr);
    animate_scanline(&mut i2c, oled_addr, &mut led).await;
}

fn scan_qwiic_bus(i2c: &mut BlockingI2c) -> Option<u8> {
    let mut oled_addr = None;

    info!("Scanning QWIIC I2C bus...");
    for addr in 0x08..0x78 {
        if i2c.blocking_write(addr, &[]).is_ok() {
            info!("Found QWIIC device at 0x{:02x}", addr);
            if addr == OLED_ADDR_PRIMARY || addr == OLED_ADDR_SECONDARY {
                oled_addr = Some(addr);
            }
        }
    }

    oled_addr
}

fn init_sh1107(i2c: &mut BlockingI2c, addr: u8) {
    let init_commands = [
        0xae, // display off
        0xd5, 0x50, // display clock
        0xa8, 0x3f, // 1/64 multiplex
        0xd3, 0x00, // display offset
        0x40, // display start line
        0xad, 0x8b, // internal DC-DC on
        0xa0, // normal segment remap
        0xc0, // normal COM scan direction
        0xda, 0x12, // COM pins
        0x81, 0x80, // contrast
        0xd9, 0x22, // pre-charge
        0xdb, 0x35, // VCOM deselect
        0xa4, // display follows RAM
        0xa6, // normal display
        0xaf, // display on
    ];

    write_commands(i2c, addr, &init_commands);
    clear_display(i2c, addr);
}

fn write_commands(i2c: &mut BlockingI2c, addr: u8, commands: &[u8]) {
    for command in commands {
        match i2c.blocking_write(addr, &[0x00, *command]) {
            Ok(_) => {}
            Err(e) => warn!("OLED command 0x{=u8:#x} error: {:?}", *command, e),
        }
    }
}

fn write_page(i2c: &mut BlockingI2c, addr: u8, page: u8, pattern: u8) {
    let page_commands = [0xb0 | page, 0x00, 0x10];
    write_commands(i2c, addr, &page_commands);

    let mut data = [0u8; DISPLAY_WIDTH + 1];
    data[0] = 0x40;
    for byte in data[1..].iter_mut() {
        *byte = pattern;
    }

    match i2c.blocking_write(addr, &data) {
        Ok(_) => {}
        Err(e) => warn!("OLED page {} data error: {:?}", page, e),
    }
}

fn clear_display(i2c: &mut BlockingI2c, addr: u8) {
    for page in 0..DISPLAY_PAGES {
        write_page(i2c, addr, page, 0x00);
    }
}

async fn animate_scanline(i2c: &mut BlockingI2c, addr: u8, led: &mut Output<'static>) -> ! {
    let mut previous_page = DISPLAY_PAGES;

    loop {
        for row in 0..DISPLAY_HEIGHT {
            let page = row / PAGE_HEIGHT;
            let bit = 1 << (row % PAGE_HEIGHT);

            led.set_high();
            if previous_page != page && previous_page < DISPLAY_PAGES {
                write_page(i2c, addr, previous_page, 0x00);
            }
            write_page(i2c, addr, page, bit);
            led.set_low();

            previous_page = page;
            Timer::after_millis(60).await;
        }
    }
}
