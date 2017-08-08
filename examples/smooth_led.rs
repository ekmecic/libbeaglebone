extern crate libbeaglebone;

use libbeaglebone::prelude::*;
use std::thread;
use std::time::Duration;

fn main() {
  // Create a PWM device at using PWM chip #0 and PWM #0, which is P9.22.
  // Note, `config-pin P9.22 pwm` has to be called prior to execution.
  // Then, export, set the period, and enable the PWM.
  let mut led = PWM::new(0, 0);
  led.set_export(DeviceState::Exported).unwrap();
  led.set_period(500_000).unwrap();
  led.set_state(PWMState::Enabled).unwrap();

  // Smoothly increase the brightness of the LED by slowly increasing the PWM
  // duty cycle in 1% increments.
  // Note: you could also use the `set_duty_cycle()` function to precisely set
  // the duty cycle in nanoseconds.
  for i in 1..100 {
    led.write(i as f32).unwrap();
    thread::sleep(Duration::from_millis(50));
  }

  // Turn off the LED and unexport the PWM device.
  led.write(0.0).unwrap();
  led.set_export(DeviceState::Unexported).unwrap();
}
