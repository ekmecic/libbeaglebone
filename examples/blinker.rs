extern crate bb_rust;

use std::time::Duration;
use std::thread;
use bb_rust::gpio::*;

fn main() {
    // Create out LED GPIO object, export it (if it isn't already),
    // and set it as an input
    let mut led = GPIO::new(69);
    led.set_export(true).unwrap();
    led.set_direction(PinDirection::Out).unwrap();

    for _ in 1..11 {
        // Toggle the LED at a 250ms period 10 times
        led.write(PinState::High).unwrap();
        thread::sleep(Duration::from_millis(250));
        led.write(PinState::Low).unwrap();
        thread::sleep(Duration::from_millis(250));
    }
}
