//! The GPIO module.
//!
//! The BeagleBone Black has 65 GPIO pins, many of which are enabled by the
//! default device tree overlay.
//! However, these pins may need to be explicitly configured as GPIOs by using
//! the `config-pin` command.
//!
//! As an example, use the following command to set GPIO 29 :
//! `sudo config-pin P9.22 gpio"`
//! The above example configure the 22nd pin on P9 (the second header), also
//! known as GPIO_2, as a GPIO.
//! This command works for recent kernel versions.

//! If you wish to configure another pin as a GPIO, substitute it's identifier
//! for P9.22.
//! A convenient list of pin identifiers can be found through an online search
//! of "BeagleBone pinout".
//!
//! Note: not all pins are available to be set as GPIOs with the default device
//! tree overlay, as they are used for other interfaces such as HDMI.
//! You may need to change the overlay from the default to access these blocked
//! pins.

use enums::DeviceState;
use errors::*;
use pins::Pin;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use util::*;

/// The direction of the pin, which can be either an input or output.
#[derive(Debug, PartialEq, Eq)]
pub enum PinDirection {
  /// GPIO in
  In,
  /// GPIO out
  Out,
}

/// The logic level of an output GPIO pin, either high or low.
#[derive(Debug, PartialEq, Eq)]
pub enum PinState {
  /// GPIO logic high
  High,
  /// GPIO logic low
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
  /// `set_direction()`, `set_export()`, yourself.
  ///
  /// Furthermore, you will need to configure the selected pin as a GPIO
  /// prior to use using the `config-pin` utility.
  /// For example, `config-pin P9.22 GPIO`.
  /// See the `examples/` directory and module documentation for more help.
  ///
  /// # Examples
  ///
  /// ```
  /// use libbeaglebone::prelude::*;
  ///
  /// // Create a new GPIO object using pin #45
  /// let pin = GPIO::new(GPIO_P8_11);
  /// ```
  ///
  /// # Errors
  ///
  /// Fails if the `pin_num` is invalid, i.e. a nonexistent pin.
  pub fn new(pin: Pin) -> GPIO {
    GPIO {
      pin_num: pin as u8,
      pin_path: PathBuf::from(format!("/sys/class/gpio/gpio{}", pin as u8)),
    }
  }

  /// Sets the direction of the pin as either an input or output.
  ///
  /// # Examples
  ///
  /// ```no_run
  /// use libbeaglebone::prelude::*;
  ///
  /// let pin = GPIO::new(GPIO_P8_11);
  ///
  /// // Make the pin an output
  /// pin.set_direction(PinDirection::Out).unwrap();
  ///
  /// // Make the in an input
  /// pin.set_direction(PinDirection::In).unwrap();
  /// ```
  ///
  /// # Errors
  ///
  /// Fails if the GPIO pin is not configured correctly.
  /// Check the module documentation to see how to configure the pin correctly.
  pub fn set_direction(&self, direction: PinDirection) -> Result<()> {
    // Write "in" or "out" to the sysfs device file depending on PinDirection
    let path = format!("/sys/class/gpio/gpio{}/direction", &self.pin_num);
    path.write_file(match direction {
      PinDirection::In => "in",
      PinDirection::Out => "out",
    })
        .chain_err(|| {
      format!("Failed to set GPIO pin #{} direction", &self.pin_num)
    })?;
    Ok(())
  }

  /// Exports or unexports a GPIO pin.
  ///
  /// True corresponds to export, false corresponds to unexport.
  /// `set_export()` won't try to export pins that are already exported, or
  /// unexport pins that aren't exported.
  ///
  /// # Examples
  ///
  /// ```no_run
  /// use libbeaglebone::prelude::*;
  ///
  /// let mut pin = GPIO::new(GPIO_P8_11);
  ///
  /// // Try to export the pin
  /// pin.set_export(DeviceState::Exported).unwrap();
  ///
  /// // Try to unexport the pin
  /// pin.set_export(DeviceState::Unexported).unwrap();
  /// ```
  ///
  /// # Errors
  ///
  /// Fails to export the pin if it isn't configured correctly.
  /// Check the module documentation to see how to configure the pin correctly.
  pub fn set_export(&self, state: DeviceState) -> Result<()> {
    // Note: if the pin path exists, the pin is already exported.
    // If the pin path doesn't exist, the pin isn't exported.
    // Exporting/unexporting is done by writing the pin number to the
    // export/unexport file.

    // The pin path doesn't exist and we want to export, try to write to the file
    if state == DeviceState::Exported && !self.pin_path.exists() {
      File::create("/sys/class/gpio/export")
        .chain_err(|| "Failed to open GPIO export file")?
        .write_all(self.pin_num.to_string().as_bytes())
        .chain_err(|| format!("Failed to export GPIO pin #{}", &self.pin_num))?;

    }
    // Try to unexport if the path exists, otherwise the pin is unexported and there's nothing to do
    else if state == DeviceState::Unexported && self.pin_path.exists() {
      File::create("/sys/class/gpio/unexport")
        .chain_err(|| "Failed to open GPIO unexport file")?
        .write_all(self.pin_num.to_string().as_bytes())
        .chain_err(|| format!("Failed to unexport GPIO pin #{}", &self.pin_num))?;
    }
    Ok(())
  }

  /// Writes to the pin, setting it either logic high or low.
  ///
  /// # Examples
  ///
  /// ```no_run
  /// use libbeaglebone::prelude::*;
  ///
  /// let mut pin = GPIO::new(GPIO_P8_11);
  ///
  /// // Try to export the pin and make it an output
  /// pin.set_export(DeviceState::Exported).unwrap();
  /// pin.set_direction(PinDirection::Out).unwrap();
  ///
  /// // Set the pin to logic high
  /// pin.write(PinState::High).unwrap();
  ///
  /// // Set the pin to logic low
  /// pin.write(PinState::High).unwrap();
  /// ```
  ///
  /// # Errors
  ///
  /// Fails to write to the pin if the pin isn't configured correctly.
  /// Check the module documentation to see how to configure the pin correctly.
  pub fn write(&mut self, state: PinState) -> Result<()> {
    let path = format!("/sys/class/gpio/gpio{}/value", &self.pin_num);
    // Write a "0" or "1" to the pin's "value" device file depending on PinState
    path.write_file(match state {
      PinState::High => "1",
      PinState::Low => "0",
    })
        .chain_err(|| {
      format!(
        "Failed to set GPIO pin #{} state to {:?}",
        &self.pin_num,
        state
      )
    })?;
    Ok(())
  }

  /// Reads the logic level of the pin, returning either high or low.
  ///
  /// # Examples
  ///
  /// ```no_run
  /// use libbeaglebone::prelude::*;
  ///
  /// let mut pin = GPIO::new(GPIO_P8_11);
  ///
  /// // Try to export the pin and make it an input
  /// pin.set_export(DeviceState::Exported).unwrap();
  /// pin.set_direction(PinDirection::In).unwrap();
  ///
  /// // Read the pin's state
  /// if pin.read().unwrap() == PinState::High {
  ///   println!("Pin is high!");
  /// }
  /// ```
  ///
  /// # Errors
  ///
  /// Fails to read from the pin if the pin isn't configured correctly.
  /// Check the module documentation to see how to configure the pin correctly.
  pub fn read(&self) -> Result<(PinState)> {
    let path = format!("/sys/class/gpio/gpio{}/value", &self.pin_num);
    // Read from the file and match the resulting bool to a PinState
    match path.read_file().unwrap().trim() {
      "1" => Ok(PinState::High),
      "0" => Ok(PinState::Low),
      _ => bail!(format!("Invalid value read from file {}", &path)),
    }
  }
}
