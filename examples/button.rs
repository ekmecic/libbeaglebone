extern crate libbeaglebone;

use libbeaglebone::prelude::*;

fn main() {
  // Create a GPIO object at pin #66 that'll represent the button, export it,
  // and set it as an input
  // Adjust the pin number to whatever pin your LED is connected to
  let button = GPIO::new(GPIO_P8_7);
  button.set_export(DeviceState::Exported).unwrap();
  button.set_direction(PinDirection::In).unwrap();
  println!("Waiting for button press...");

  for _ in 1..6 {
    // Wait for button to be hit and then released 5 times
    while button.read().unwrap() != PinState::High {}
    println!("Button hit!");
    while button.read().unwrap() != PinState::Low {}
    println!("Button released!");
  }

  // Unexport the button once we're done with it.
  button.set_export(DeviceState::Unexported).unwrap();
}
