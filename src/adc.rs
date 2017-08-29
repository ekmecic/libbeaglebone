//! The ADC module.
//!
//! The BeagleBone Black has 7 usable ADC inputs.
//!
//! These can be enabled by using the bone_capemgr module and enabling a device
//! tree overlay that allows the use of the ADCs.
//! One such overlay is the BB-ADC overlay, which can be enabled with the
//! following command:
//! `sudo sh -c "echo 'BB-ADC' > /sys/devices/platform/bone_capemgr/slots"`
//! This command works for recent kernel versions.
//!
//! *NOTE:* the ADC inputs on the BeagleBone are limited to 1.8V.
//! Be careful not to exceed this limit or you may damage the BeagleBone (don't
//! ask me how I know that!).

use errors::*;
use pins::Pin;
use util::*;

/// Represents a pin configured as an ADC.
#[derive(Debug)]
pub struct ADC {
  adc_num: u16,
  scaling_factor: f32,
}

impl ADC {
  /// Creates a new ADC object.
  pub fn new(pin: Pin, scaling_factor: f32) -> ADC {
    ADC {
      adc_num: (pin as u16) - 1000,
      scaling_factor: scaling_factor,
    }
  }

  /// Reads the raw voltage of the ADC.
  ///
  /// # Examples
  ///
  /// ```no_run
  /// use libbeaglebone::prelude::*;
  ///
  /// // Create a new ADC pin using ADC #0 and no scaling factor.
  /// let mut sensor = ADC::new(AIN_0, 0.0);
  ///
  /// // Read the ADC value.
  /// sensor.read().unwrap();
  /// ```
  pub fn read(&self) -> Result<u32> {
    let path = format!(
      "/sys/bus/iio/devices/iio:device0/in_voltage{}_raw",
      &self.adc_num
    );

    Ok(
      path.read_file()
          .chain_err(|| format!("Failed to read from ADC #{}", &self.adc_num))?
          .trim()
          .to_string()
          .parse::<u32>()
          .chain_err(|| format!("Failed to parse ADC #{} value", &self.adc_num))?,
    )
  }

  /// Reads the raw voltage of the ADC and applies a scaling factor to it.
  ///
  /// Useful for converting from a raw voltage to the actual unit measured by a
  /// sensor.
  /// For example, raw voltage -> degrees Celsius for a temperature sensor
  /// might have a scaling factor of 0.0122.
  ///
  /// # Examples
  ///
  /// ```no_run
  /// use libbeaglebone::prelude::*;
  ///
  /// // Create a new ADC pin using ADC #6 and a scaling factor of 0.0122.
  /// let mut sensor = ADC::new(AIN_6, 0.0122);
  ///
  /// // Read the ADC value and scale it to degrees Celsius.
  /// sensor.scaled_read().unwrap();
  /// ```
  pub fn scaled_read(&self) -> Result<f32> {
    let path = format!(
      "/sys/bus/iio/devices/iio:device0/in_voltage{}_raw",
      &self.adc_num
    );

    let raw_value =
      path.read_file()
          .chain_err(|| format!("Failed to read from ADC #{}", &self.adc_num))?
          .trim()
          .to_string()
          .parse::<u32>()
          .chain_err(|| format!("Failed to parse ADC #{} value", &self.adc_num))?;

    Ok(raw_value as f32 * self.scaling_factor)
  }
}
