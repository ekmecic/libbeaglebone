//! The GPIO module

use errors::*;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use util::*;

/// The direction a pin works in
#[derive(Debug, PartialEq, Eq)]
pub enum PinDirection {
  /// GPIO out, self explanatory
  In,
  /// GPIO out, self explanatory
  Out,
}

/// The logic level of an output GPIO pin
#[derive(Debug, PartialEq, Eq)]
pub enum PinState {
  /// GPIO out, self explanatory
  High,
  /// GPIO out, self explanatory
  Low,
}

/// Represents a pin configured as a GPIO
#[derive(Debug)]
pub struct GPIO {
  pin_num: u8,
  pin_path: PathBuf,
}

impl GPIO {
  /// Creates a new GPIO pin
  pub fn new(m_pin_num: u8) -> GPIO {
    let m_pin_path = format!("/sys/class/gpio/gpio{}", m_pin_num);
    GPIO {
      pin_num: m_pin_num,
      pin_path: PathBuf::from(m_pin_path),
    }
  }

  /// Sets the direction of the pin
  pub fn set_direction(&self, direction: PinDirection) -> Result<()> {
    write_file(match direction {
                 PinDirection::In => "in",
                 PinDirection::Out => "out",
               },
               "direction",
               &self.pin_num)
      .chain_err(|| format!("Failed to set GPIO pin #{} direction", &self.pin_num))?;
    Ok(())
  }

  /// Export or unexport a GPIO
  pub fn set_export(&self, state: bool) -> Result<()> {
    // If the path exists, try to unexport it
    // If it doesn't exist, don't do anything
    if state && !self.pin_path.exists() {
      File::create("/sys/class/gpio/export")
        .chain_err(|| "Failed to open GPIO export file")?
        .write_all(self.pin_num.to_string().as_bytes())
        .chain_err(|| format!("Failed to export GPIO pin #{}", &self.pin_num))?;

    } else if !state && self.pin_path.exists() {
      File::create("/sys/class/gpio/unexport")
        .chain_err(|| "Failed to open GPIO unexport file")?
        .write_all(self.pin_num.to_string().as_bytes())
        .chain_err(|| format!("Failed to unexport GPIO pin #{}", &self.pin_num))?;
    }
    Ok(())
  }

  /// Set the state hi or lo
  pub fn write(&mut self, state: PinState) -> Result<()> {
    write_file(match state {
                 PinState::High => "1",
                 PinState::Low => "0",
               },
               "value",
               &self.pin_num)
      .chain_err(|| {
                   format!("Failed to set GPIO pin #{} state to {}",
                           &self.pin_num,
                           state as u8)
                 })?;
    Ok(())
  }

  /// Get the state of the pin
  pub fn read(&self) -> Result<(PinState)> {
    if read_file("value", &self.pin_num).unwrap() {
      Ok(PinState::High)
    } else {
      Ok(PinState::Low)
    }
  }
}
