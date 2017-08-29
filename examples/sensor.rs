extern crate libbeaglebone;

use libbeaglebone::prelude::*;
use std::thread;
use std::time::Duration;

fn main() {
  // Create a new ADC object using AIN-0 and a scaling factor of 0.
  let sensor = ADC::new(AIN_0, 0.0);

  // Read from the ADC object every 50ms 100 times, and print out the value.
  for _ in 1..101 {
    println!("{}", sensor.read().unwrap());
    thread::sleep(Duration::from_millis(50));
  }
}
