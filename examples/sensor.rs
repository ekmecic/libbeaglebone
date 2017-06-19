extern crate libbeaglebone;

use libbeaglebone::adc::ADC;
use std::thread;
use std::time::Duration;

fn main() {
  let sensor = ADC::new(0, 0.0);

  for _ in 1..101 {
    println!("{}", sensor.read().unwrap());
    thread::sleep(Duration::from_millis(50));
  }
}
