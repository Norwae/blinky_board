#![no_std]
#![no_main]

// Pull in the panic handler from panic-halt
extern crate panic_halt;

use arduino_uno::prelude::*;
use arduino_uno::{Pins, delay_ms};

#[arduino_uno::entry]
fn main() -> ! {
    let peripherials = arduino_uno::Peripherals::take().unwrap();
    let pins = Pins::new(peripherials.PORTB, peripherials.PORTC, peripherials.PORTD);
    let mut led1_out = pins.d13.into_output(&pins.ddr);
    let mut led2_out = pins.d12.into_output(&pins.ddr);
    let mut delay = 0u16;
    const MAX_DELAY: u16 = 1000;

    led1_out.toggle().unwrap();
    loop {
        delay_ms(delay);
        led1_out.toggle().unwrap();
        led2_out.toggle().unwrap();

        delay = if delay < MAX_DELAY { delay + 100 } else { 0 }
    }
}