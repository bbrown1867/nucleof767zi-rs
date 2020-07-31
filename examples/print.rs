#![no_main]
#![no_std]

extern crate panic_halt;

use cortex_m_rt::entry;
use nucleo144_rs::{uprint, uprintln, StLinkSerial};
use stm32f7xx_hal::{delay::Delay, device, prelude::*};

use core::fmt::Write;

#[entry]
fn main() -> ! {
    let pac_periph = device::Peripherals::take().unwrap();
    let cm_periph = cortex_m::Peripherals::take().unwrap();

    let rcc = pac_periph.RCC.constrain();
    let clocks = rcc.cfgr.sysclk(216.mhz()).freeze();
    let mut delay = Delay::new(cm_periph.SYST, clocks);

    let usart3 = pac_periph.USART3;
    let gpiod = pac_periph.GPIOD.split();
    let mut stlink_serial = StLinkSerial::new(gpiod, usart3, clocks);

    loop {
        uprintln!(stlink_serial, "Hello World!\r\n");
        delay.delay_ms(500_u16);
    }
}
