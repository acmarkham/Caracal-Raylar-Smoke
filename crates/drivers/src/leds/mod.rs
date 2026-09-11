//! Logical-name LED driver for board-owned GPIO outputs.

#![cfg(feature = "stm32")]

use embassy_stm32::gpio::Output;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum LedName { SysGpsGreen, SysGpsRed, SysMainRed, SysMainGreen, SysSdBlue }

pub struct LedResources<'d> {
    pub sys_gps_green: Output<'d>,
    pub sys_gps_red: Output<'d>,
    pub sys_main_red: Output<'d>,
    pub sys_main_green: Output<'d>,
    pub sys_sd_blue: Output<'d>,
}

pub struct LedDriver<'d> { resources: LedResources<'d> }

pub fn init(resources: LedResources<'static>) -> LedDriver<'static> { LedDriver { resources } }

impl<'d> LedDriver<'d> {
    pub fn on(&mut self, led: LedName) { self.output(led).set_high(); }
    pub fn off(&mut self, led: LedName) { self.output(led).set_low(); }
    pub fn toggle(&mut self, led: LedName) { self.output(led).toggle(); }

    fn output(&mut self, led: LedName) -> &mut Output<'d> {
        match led {
            LedName::SysGpsGreen => &mut self.resources.sys_gps_green,
            LedName::SysGpsRed => &mut self.resources.sys_gps_red,
            LedName::SysMainRed => &mut self.resources.sys_main_red,
            LedName::SysMainGreen => &mut self.resources.sys_main_green,
            LedName::SysSdBlue => &mut self.resources.sys_sd_blue,
        }
    }
}
