//! The GPIO module.

use errors::*;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use util::*;

/// The direction of the pin, which can be either an input or output.
#[derive(Debug, PartialEq, Eq)]
pub enum PinDirection {
  /// GPIO out, self explanatory
  In,
  /// GPIO out, self explanatory
  Out,
}

/// The logic level of an output GPIO pin, either high or low.
#[derive(Debug, PartialEq, Eq)]
pub enum PinState {
  /// GPIO out, self explanatory
  High,
  /// GPIO out, self explanatory
  Low,
}

/// Represents a pin configured as a GPIO.
#[derive(Debug)]
pub struct GPIO {
  pin_num: u8,
  pin_path: PathBuf,
}

impl GPIO {
  /// Creates a new GPIO pin object.
  ///
  /// Note: this doesn't do any sort of initialization, you have to call
  /// `set_direction()`, `set_export()`, youself.
  ///
  /// # Examples
  ///
  /// ```
  /// use libbeaglebone::gpio::GPIO;
  ///
  /// // Create a new GPIO object using pin #45
  /// let mut pin = GPIO::new(45);
  /// ```
  pub fn new(m_pin_num: u8) -> GPIO {
    let m_pin_path = format!("/sys/class/gpio/gpio{}", m_pin_num);
    GPIO {
      pin_num: m_pin_num,
      pin_path: PathBuf::from(m_pin_path),
    }
  }

  /// Sets the direction of the pin as either an input or output.
  ///
  /// # Examples
  ///
  /// ```rust,no_run
  /// use libbeaglebone::gpio::{GPIO, PinDirection};
  ///
  /// let pin = GPIO::new(45);
  ///
  /// // Make the pin an output
  /// pin.set_direction(PinDirection::Out).unwrap();
  ///
  /// // Make the in an input
  /// pin.set_direction(PinDirection::In).unwrap();
  /// ```
  pub fn set_direction(&self, direction: PinDirection) -> Result<()> {
    // Write "in" or "out" to the sysfs device file depending on PinDirection
    write_file(match direction {
                 PinDirection::In => "in",
                 PinDirection::Out => "out",
               },
               "direction",
               &self.pin_num)
      .chain_err(|| format!("Failed to set GPIO pin #{} direction", &self.pin_num))?;
    Ok(())
  }

  /// Exports or unexports a GPIO pin.
  ///
  /// True corresponds to export, false corresponds to unexport.
  /// `set_export()` won't try to export pins that are already exported.
  ///
  /// # Examples
  ///
  /// ```rust,no_run
  /// use libbeaglebone::gpio::{GPIO};
  ///
  /// let mut pin = GPIO::new(45);
  ///
  /// // Try to export the pin
  /// pin.set_export(true).unwrap();
  ///
  /// // Try to unexport the pin
  /// pin.set_export(false).unwrap();
  /// ```
  pub fn set_export(&self, state: bool) -> Result<()> {
    // Note: if the pin path exists, the pin is already exported.
    // If the pin path doesn't exist, the pin isn't exported.
    // Exporting/unexporting is done by writing the pin number to the
    // export/unexport file.

    // The pin path doesn't exist and we want to export, try to write to the file
    if state && !self.pin_path.exists() {
      File::create("/sys/class/gpio/export")
        .chain_err(|| "Failed to open GPIO export file")?
        .write_all(self.pin_num.to_string().as_bytes())
        .chain_err(|| format!("Failed to export GPIO pin #{}", &self.pin_num))?;

    }
    // Try to unexport if the path exists, otherwise the pin is unexported and there's nothing to do
    else if !state && self.pin_path.exists() {
      File::create("/sys/class/gpio/unexport")
        .chain_err(|| "Failed to open GPIO unexport file")?
        .write_all(self.pin_num.to_string().as_bytes())
        .chain_err(|| format!("Failed to unexport GPIO pin #{}", &self.pin_num))?;
    }
    Ok(())
  }

  /// Writes to the pin, setting it either logic high or low.
  ///
  ///
  /// # Examples
  ///
  /// ```rust,no_run
  /// use libbeaglebone::gpio::{GPIO, PinState, PinDirection};
  ///
  /// let mut pin = GPIO::new(45);
  ///
  /// // Try to export the pin and make it an output
  /// pin.set_export(true).unwrap();
  /// pin.set_direction(PinDirection::Out).unwrap();
  ///
  /// // Set the pin to logic high
  /// pin.write(PinState::High).unwrap();
  ///
  /// // Set the pin to logic low
  /// pin.write(PinState::High).unwrap();
  /// ```
  pub fn write(&mut self, state: PinState) -> Result<()> {
    // Write a "0" or "1" to the pin's "value" device file depending on PinState
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

  /// Reads the logic level of the pin, returning either high or low.
  ///
  /// # Examples
  ///
  /// ```rust,no_run
  /// use libbeaglebone::gpio::{GPIO, PinDirection};
  ///
  /// let mut pin = GPIO::new(45);
  ///
  /// // Try to export the pin and make it an input
  /// pin.set_export(true).unwrap();
  /// pin.set_direction(PinDirection::In).unwrap();
  ///
  /// // Read the pin's state
  /// pin.read().unwrap();
  /// ```
  pub fn read(&self) -> Result<(PinState)> {
    // Read from the file and match the resulting bool to a PinState
    if read_file("value", &self.pin_num).unwrap() {
      Ok(PinState::High)
    } else {
      Ok(PinState::Low)
    }
  }
}
