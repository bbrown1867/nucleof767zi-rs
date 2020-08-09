#![no_std]

use cortex_m::peripheral::NVIC;

use stm32f7xx_hal::device::{self, EXTI, SYSCFG, USART3};
use stm32f7xx_hal::gpio::gpiob::PB;
use stm32f7xx_hal::gpio::gpioc::PC13;
use stm32f7xx_hal::gpio::{gpiob, gpioc, gpiod, Edge, ExtiPin, Floating, Input, Output, PushPull};
use stm32f7xx_hal::interrupt;
use stm32f7xx_hal::rcc::Clocks;
use stm32f7xx_hal::serial::{self, Serial, Tx};

use embedded_hal::digital::v2::InputPin;
use embedded_hal::digital::v2::OutputPin;

// Push Button

static mut USER_BUTTON: Option<UserButton> = None;

pub struct UserButton {
    callback: fn(&mut UserButton),
    pin: PC13<Input<Floating>>,
    // It may be useful for the user to know the counter in the callback
    pub isr_count: u32,
    // LED1 will be initialized in setup to use in the callback
    pub debug_led: Led,
}

impl UserButton {
    pub fn setup(
        callback: fn(&mut UserButton),
        gpiob: gpiob::Parts,
        gpioc: gpioc::Parts,
        syscfg: &mut SYSCFG,
        exti: &mut EXTI,
    ) {
        // TODO: Bug in HAL
        const SYSCFG_EN: u32 = 14;
        unsafe {
            &(*device::RCC::ptr())
                .apb2enr
                .modify(|r, w| w.bits(r.bits() | (1 << SYSCFG_EN)));
        }

        // Debug LED configuration
        let led1_pin = gpiob.pb0.into_push_pull_output().downgrade();
        let led1 = Led::new(led1_pin);

        // Push button configuration;
        let mut button_pin = gpioc.pc13.into_floating_input();
        button_pin.make_interrupt_source(syscfg);
        button_pin.trigger_on_edge(exti, Edge::RISING);
        button_pin.enable_interrupt(exti);
        unsafe {
            NVIC::unmask::<interrupt>(interrupt::EXTI15_10);
        }

        // Save the global state, such that ISR can work
        unsafe {
            USER_BUTTON = Some(UserButton {
                callback,
                pin: button_pin,
                isr_count: 0,
                debug_led: led1,
            });
        }
    }
}

#[interrupt]
fn EXTI15_10() {
    unsafe {
        match &mut USER_BUTTON {
            Some(button) => {
                // Clear the push button interrupt
                button.pin.clear_interrupt_pending_bit();

                // Call the callback
                (button.callback)(button);

                button.isr_count += 1;
            }
            None => (),
        }
    }
}

// Serial

#[macro_export]
macro_rules! uprint {
    ($serial:expr, $($arg:tt)*) => {
        $serial.write_fmt(format_args!($($arg)*)).ok()
    };
}

#[macro_export]
macro_rules! uprintln {
    ($serial:expr, $fmt:expr) => {
        uprint!($serial, concat!($fmt, "\n"))
    };
    ($serial:expr, $fmt:expr, $($arg:tt)*) => {
        uprint!($serial, concat!($fmt, "\n"), $($arg)*)
    };
}

pub struct StLinkSerial {
    tx: Tx<USART3>,
}

impl StLinkSerial {
    pub fn new(gpiod: gpiod::Parts, usart: USART3, clocks: Clocks) -> Self {
        let tx_pin = gpiod.pd8.into_alternate_af7();
        let rx_pin = gpiod.pd9.into_alternate_af7();

        let sp = Serial::new(usart, (tx_pin, rx_pin), clocks, serial::Config::default());

        let (tx, _) = sp.split();

        StLinkSerial { tx }
    }
}

impl core::fmt::Write for StLinkSerial {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        self.tx.write_str(s)
    }
}

// LED

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

    pub fn on(&mut self) -> () {
        self.pin.set_high().ok();
    }

    pub fn toggle(&mut self) -> () {
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
