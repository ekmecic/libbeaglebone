extern crate libbeaglebone;

use libbeaglebone::prelude::*;
use std::thread;
use std::time::Duration;

fn main() {
  // Create a GPIO object at pin #69 that'll represent the LED, export it, and
  // set it as an output
  // Adjust the pin number to whatever pin your LED is connected to
  let mut led = GPIO::new(GPIO_P8_9);
  led.set_export(DeviceState::Exported).unwrap();
  led.set_direction(PinDirection::Out).unwrap();

  for _ in 1..11 {
    // Toggle the LED on and off every 250ms 10 times
    led.write(PinState::High).unwrap();
    thread::sleep(Duration::from_millis(250));
    led.write(PinState::Low).unwrap();
    thread::sleep(Duration::from_millis(250));
  }

  // Unexport the LED once we're done with it.
  led.set_export(DeviceState::Unexported).unwrap();
}
