#![no_main]
#![no_std]

extern crate panic_halt;

use cortex_m_rt::entry;
use stm32f7xx_hal::{
    device,
    delay::Delay,
    prelude::*
};

#[entry]
fn main() -> ! {
    let p = device::Peripherals::take().unwrap();
    let cp = cortex_m::Peripherals::take().unwrap();

    let rcc = p.RCC.constrain();
    let clocks = rcc.cfgr.sysclk(216.mhz()).freeze();
    let mut delay = Delay::new(cp.SYST, clocks);

    let gpiob = p.GPIOB.split();
    let mut led1 = gpiob.pb0.into_push_pull_output();
    let mut led2 = gpiob.pb7.into_push_pull_output();
    let mut led3 = gpiob.pb14.into_push_pull_output();

    loop {
        led1.set_high().expect("GPIO can never fail");
        led2.set_high().expect("GPIO can never fail");
        led3.set_high().expect("GPIO can never fail");
        delay.delay_ms(1000_u16);

        led1.set_low().expect("GPIO can never fail");
        led2.set_low().expect("GPIO can never fail");
        led3.set_low().expect("GPIO can never fail");
        delay.delay_ms(1000_u16);
    }
}
