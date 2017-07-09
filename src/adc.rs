//! The ADC module.

use errors::*;
use util::*;

/// Represents a pin configured as an ADC.
#[derive(Debug)]
pub struct ADC {
  adc_num: u8,
  scaling_factor: f32,
}

impl ADC {
  /// Creates a new ADC object.
  pub fn new(m_adc_num: u8, m_scaling_factor: f32) -> ADC {
    ADC {
      adc_num: m_adc_num,
      scaling_factor: m_scaling_factor,
    }
  }

  /// Reads the raw voltage of the ADC.
  ///
  /// # Examples
  ///
  /// ```no_run
  /// use libbeaglebone::adc::ADC;
  ///
  /// // Create a new ADC pin using ADC #0 and no scaling factor.
  /// let mut sensor = ADC::new(0, 0.0);
  ///
  /// // Read the ADC value.
  /// sensor.read().unwrap();
  /// ```
  pub fn read(&self) -> Result<u32> {
    let path = format!("/sys/bus/iio/devices/iio:device0/in_voltage{}_raw",
                       &self.adc_num);

    Ok(read_file(&path)
         .chain_err(|| format!("Failed to read from ADC #{}", &self.adc_num))?
         .trim()
         .to_string()
         .parse::<u32>()
         .chain_err(|| format!("Failed to parse ADC #{} value", &self.adc_num))?)
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
  /// use libbeaglebone::adc::ADC;
  ///
  /// // Create a new ADC pin using ADC #6 and a scaling factor of 0.0122.
  /// let mut sensor = ADC::new(6, 0.0122);
  ///
  /// // Read the ADC value and scale it to degrees Celsius.
  /// sensor.scaled_read().unwrap();
  /// ```
  pub fn scaled_read(&self) -> Result<f32> {
    let path = format!("/sys/bus/iio/devices/iio:device0/in_voltage{}_raw",
                       &self.adc_num);

    let raw_value = read_file(&path)
      .chain_err(|| format!("Failed to read from ADC #{}", &self.adc_num))?
      .trim()
      .to_string()
      .parse::<u32>()
      .chain_err(|| format!("Failed to parse ADC #{} value", &self.adc_num))?;

    Ok(raw_value as f32 * self.scaling_factor)
  }
}
