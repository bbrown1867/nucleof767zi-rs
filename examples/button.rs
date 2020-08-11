#![no_main]
#![no_std]

extern crate panic_halt;

use core::cell::Cell;
use cortex_m::interrupt::{free, Mutex};
use cortex_m_rt::entry;
use nucleof767zi_rs::{Led, UserButton};
use stm32f7xx_hal::{device, prelude::*};

// Semaphore for synchronization
static SEMAPHORE: Mutex<Cell<bool>> = Mutex::new(Cell::new(true));

#[entry]
fn main() -> ! {
    let pac_periph = device::Peripherals::take().unwrap();

    // Debug LED configuration
    let gpiob = pac_periph.GPIOB.split();
    let led1_pin = gpiob.pb0.into_push_pull_output().downgrade();
    let mut led1 = Led::new(led1_pin);

    // Push button configuration
    let gpioc = pac_periph.GPIOC.split();
    let mut syscfg = pac_periph.SYSCFG;
    let mut exti = pac_periph.EXTI;
    UserButton::setup(button_cb, gpioc, &mut syscfg, &mut exti);

    // Freeze clocks
    let rcc = pac_periph.RCC.constrain();
    rcc.cfgr.sysclk(216.mhz()).freeze();

    loop {
        // Wait for interrupt to fire
        free(|cs| {
            if SEMAPHORE.borrow(cs).get() == false {
                // Toggle debug LED
                led1.toggle();
                SEMAPHORE.borrow(cs).set(true);
            }
        });
    }
}

fn button_cb() {
    // Signal that interrupt fired
    free(|cs| SEMAPHORE.borrow(cs).set(false));
}
