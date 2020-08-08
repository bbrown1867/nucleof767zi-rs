#![no_main]
#![no_std]

extern crate panic_halt;

use cortex_m::peripheral::NVIC;
use cortex_m_rt::entry;
use nucleof767zi_rs::Led;
use stm32f7xx_hal::gpio::gpioc::PC13;
use stm32f7xx_hal::gpio::{Edge, ExtiPin, Floating, Input};
use stm32f7xx_hal::{device, interrupt, prelude::*};

static mut BUTTON: Option<PC13<Input<Floating>>> = None;
static mut LED: Option<Led> = None;

#[entry]
fn main() -> ! {
    let pac_periph = device::Peripherals::take().unwrap();
    let rcc = pac_periph.RCC.constrain();
    rcc.cfgr.sysclk(216.mhz()).freeze();

    // Push button
    let mut syscfg = pac_periph.SYSCFG;
    let mut exti = pac_periph.EXTI;
    let gpioc = pac_periph.GPIOC.split();
    unsafe {
        BUTTON = Some(gpioc.pc13.into_floating_input());

        match &mut BUTTON {
            Some(button) => {
                button.make_interrupt_source(&mut syscfg);

                // syscfg.exticr4.modify(|r, w|
                //     w.bits((r.bits() & !(0xf << 8)) | (0x2 << 8))
                // );

                button.trigger_on_edge(&mut exti, Edge::RISING);
                button.enable_interrupt(&mut exti);
                NVIC::unmask::<interrupt>(interrupt::EXTI15_10);
            }
            None => panic!("Error initializing button.\r\n"),
        }
    }

    // Debug LED
    let gpiob = pac_periph.GPIOB.split();
    unsafe {
        LED = Some(Led::new(gpiob.pb0.into_push_pull_output().downgrade()));

        match &mut LED {
            Some(led) => led.on(),
            None => panic!("Error initializing LED.\r\n"),
        };
    }

    loop {}
}

#[interrupt]
fn EXTI15_10() {
    unsafe {
        match &mut BUTTON {
            Some(button) => button.clear_interrupt_pending_bit(),
            None => panic!("Error initializing button.\r\n"),
        };

        match &mut LED {
            Some(led) => led.toggle(),
            None => panic!("Error initializing LED.\r\n"),
        };
    }
}
