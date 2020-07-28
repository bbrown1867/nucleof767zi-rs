#![no_main]
#![no_std]

extern crate panic_semihosting;

use cortex_m_rt::entry;
use cortex_m_semihosting::hprintln;
use stm32f7xx_hal::{delay::Delay, device, prelude::*};
use nucleo144_rs::{Leds, LED1, LED2, LED3};

#[entry]
fn main() -> ! {
    let hal_periph = device::Peripherals::take().unwrap();
    let cm_periph = cortex_m::Peripherals::take().unwrap();

    let rcc = hal_periph.RCC.constrain();
    let clocks = rcc.cfgr.sysclk(216.mhz()).freeze();
    let mut delay = Delay::new(cm_periph.SYST, clocks);

    let gpiob = hal_periph.GPIOB.split();
    let mut leds = Leds::new(gpiob);

    loop {
        leds[LED1].toggle();
        leds[LED2].toggle();
        leds[LED3].toggle();

        delay.delay_ms(500_u16);

        hprintln!("Hello World!\r\n").unwrap();
    }
}
