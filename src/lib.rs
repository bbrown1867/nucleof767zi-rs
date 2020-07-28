#![no_std]

use stm32f7xx_hal::gpio::gpiob::{self, PB};
use stm32f7xx_hal::gpio::{Output, PushPull};

use embedded_hal::digital::v2::InputPin;
use embedded_hal::digital::v2::OutputPin;

pub const LED1: usize = 0;
pub const LED2: usize = 1;
pub const LED3: usize = 2;

pub struct Led {
    pin: PB<Output<PushPull>>,
}

impl Led {
    pub fn new(pin: PB<Output<PushPull>>) -> Self {
        Led { pin }
    }

    pub fn off(&mut self) -> () {
        self.pin.set_low().ok();
    }

    pub fn on(&mut self) {
        self.pin.set_high().ok();
    }

    pub fn toggle(&mut self) {
        if let Ok(true) = self.pin.is_low() {
            self.pin.set_high().ok();
        } else {
            self.pin.set_low().ok();
        }
    }
}

pub struct Leds {
    leds: [Led; 3],
}

impl Leds {
    pub fn new(gpiob: gpiob::Parts) -> Self {
        let led1 = gpiob.pb0.into_push_pull_output().downgrade();
        let led2 = gpiob.pb7.into_push_pull_output().downgrade();
        let led3 = gpiob.pb14.into_push_pull_output().downgrade();
        Leds {
            leds: [Led::new(led1), Led::new(led2), Led::new(led3)],
        }
    }
}

impl core::ops::Index<usize> for Leds {
    type Output = Led;

    fn index(&self, i: usize) -> &Led {
        &self.leds[i]
    }
}

impl core::ops::IndexMut<usize> for Leds {
    fn index_mut(&mut self, i: usize) -> &mut Led {
        &mut self.leds[i]
    }
}
