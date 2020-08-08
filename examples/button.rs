#![no_main]
#![no_std]

extern crate panic_halt;

use cortex_m::peripheral::NVIC;
use cortex_m_rt::entry;
use nucleof767zi_rs::Led;
use stm32f7xx_hal::gpio::{Edge, ExtiPin};
use stm32f7xx_hal::{device, interrupt, prelude::*};

#[entry]
fn main() -> ! {
    let pac_periph = device::Peripherals::take().unwrap();
    let rcc = pac_periph.RCC.constrain();
    rcc.cfgr.sysclk(216.mhz()).freeze();

    // Push button configuration
    let mut syscfg = pac_periph.SYSCFG;
    let mut exti = pac_periph.EXTI;
    let gpioc = pac_periph.GPIOC.split();
    let mut button = gpioc.pc13.into_floating_input();
    button.make_interrupt_source(&mut syscfg);
    button.trigger_on_edge(&mut exti, Edge::RISING);
    button.enable_interrupt(&mut exti);
    unsafe {
        NVIC::unmask::<interrupt>(interrupt::EXTI15_10);
    }

    // Problem appears to be that exticr4 never gets set properly, and we trigger on PA13 instead

    // syscfg.exticr4.modify(|_, w| unsafe {
    //     w.bits(0x20)
    // });
    // let _test = syscfg.exticr4.read().bits();

    // let REG_TEST: *mut u32 = 0x40013814 as *mut u32;
    // *REG_TEST = 0x20;
    // let _test = *REG_TEST;

    loop {}
}

#[interrupt]
fn EXTI15_10() {
    static mut COUNT: u32 = 0;

    unsafe {
        // TODO: Is there a safe alternative? Using a mutable static GPIO pin is also unsafe
        let pac_periph = device::Peripherals::steal();

        // Clear the push button interrupt
        let gpioc = pac_periph.GPIOC.split();
        let mut button = gpioc.pc13.into_floating_input();
        button.clear_interrupt_pending_bit();

        // Blink an LED for debug purposes
        let gpiob = pac_periph.GPIOB.split();
        let mut led1 = Led::new(gpiob.pb0.into_push_pull_output().downgrade());
        if *COUNT & 0x1 == 0x01 {
            led1.on();
        } else {
            led1.off();
        }
    }

    *COUNT += 1;
}
