#![no_main]
#![no_std]

extern crate panic_halt;

use cortex_m_rt::entry;
use nucleof767zi_rs::UserButton;
use stm32f7xx_hal::{device, prelude::*};

#[entry]
fn main() -> ! {
    let pac_periph = device::Peripherals::take().unwrap();

    let gpiob = pac_periph.GPIOB.split();
    let gpioc = pac_periph.GPIOC.split();
    let mut syscfg = pac_periph.SYSCFG;
    let mut exti = pac_periph.EXTI;
    UserButton::setup(button_cb, gpiob, gpioc, &mut syscfg, &mut exti);

    let rcc = pac_periph.RCC.constrain();
    rcc.cfgr.sysclk(216.mhz()).freeze();

    loop {}
}

fn button_cb(button: &mut UserButton) {
    if button.isr_count & 0x1 == 0x01 {
        button.debug_led.on();
    } else {
        button.debug_led.off();
    }
}
