extern crate bb_rust;

use bb_rust::gpio::*;

fn main() {
  // Create our button GPIO object, export it (if it isn't already),
  // and set it as an input.
  let button = GPIO::new(66);
  button.set_export(true).unwrap();
  button.set_direction(PinDirection::In).unwrap();
  println!("Waiting for button press...");

  for _ in 1..6 {
    // Wait for button to be hit and then released 5 times
    while button.read().unwrap() != PinState::High {}
    println!("Button hit!");
    while button.read().unwrap() != PinState::Low {}
    println!("Button released!");
  }
}
