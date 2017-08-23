//! The PWM module.
//!
//! The BeagleBone Black has 8 PWM outputs.
//! However, these pins may need to be explicitly configured as PWMs by using
//! the `config-pin` command.
//!
//! As an example, use the following command to set GPIO 29 :
//! `sudo config-pin P9.29 pwm"`
//! The above example configure the 29 pin on P9 (the second header), also
//! known as EHRPWM0B, as a PWM.
//! This command works for recent kernel versions.

//! If you wish to configure another pin as a PWM, substitute it's identifier
//! for P9.29.
//! A convenient list of pin identifiers can be found through an online search
//! of "BeagleBone pinout".

use enums::DeviceState;
use errors::*;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use util::*;

/// The state in which the PWM is in, either on or off.
#[derive(Debug, PartialEq, Eq)]
pub enum PWMState {
  /// PWM on
  Enabled,
  /// PWM off
  Disabled,
}

/// Represents a PWM device.
#[derive(Debug)]
pub struct PWM {
  pwm_chip_num: u8,
  pwm_num: u8,
  period: u32,
  duty_cycle: u32,
  state: PWMState,
}

impl PWM {
  /// Creates a new PWM object.
  ///
  /// Note: you will need to configure the selected pin as a PWM output prior
  /// to use using the `config-pin` utility.
  /// For example, `config-pin P9.21 pwm`.
  /// See the `examples/` directory and module documentation for more help.
  ///
  /// # Examples
  ///
  /// ```no_run
  /// use libbeaglebone::prelude::*;
  ///
  /// // Create a new PWM device using PWM chip 0 and PWM 0.
  /// let mut pwm = PWM::new(0, 0);
  /// ```
  ///
  /// # Errors
  ///
  /// Fails if either `pwm_chip_num` or `pwm_num` are invalid, i.e. you're
  /// trying to refer to a nonexistent PWM.
  pub fn new(pwm_chip_num: u8, pwm_num: u8) -> PWM {
    PWM {
      pwm_chip_num: pwm_chip_num,
      pwm_num: pwm_num,
      period: 0,
      duty_cycle: 0,
      state: PWMState::Disabled,
    }
  }

  /// Exports the PWM.
  ///
  /// # Examples
  ///
  /// ```no_run
  /// use libbeaglebone::prelude::*;
  ///
  /// // Create a new PWM device using PWM chip 0 and PWM 0.
  /// let mut pwm = PWM::new(0, 0);
  ///
  /// // Export the PWM.
  /// pwm.set_export(DeviceState::Exported).unwrap();
  /// ```
  ///
  /// # Errors
  ///
  /// Fails to export to the PWM if it isn't configured correctly or if the
  /// kernel refuses to execute the instruction.
  pub fn set_export(&self, state: DeviceState) -> Result<()> {
    let path = PathBuf::from(format!(
      "/sys/class/pwm/pwmchip{}/pwm{}",
      &self.pwm_chip_num,
      &self.pwm_num
    ));
    // If w're trying to export and the pin isn't already exported, try to export
    // it.
    if state == DeviceState::Exported && !path.exists() {
      File::create(format!(
        "/sys/class/pwm/pwmchip{}/export",
        &self.pwm_chip_num
      ))
      .chain_err(|| "Failed to open PWM export file")?
      .write_all(self.pwm_num.to_string().as_bytes())
      .chain_err(|| {
        format!(
          "Failed to export PWM #{}-{}",
          &self.pwm_chip_num,
          &self.pwm_num
        )
      })?;
    }
    // Try to unexport if the path exists, otherwise the device is unexported and there's nothing
    // to do.
    else if state == DeviceState::Unexported && path.exists() {
      File::create(format!(
        "/sys/class/pwm/pwmchip{}/unexport",
        &self.pwm_chip_num
      ))
      .chain_err(|| "Failed to open PWM unexport file")?
      .write_all(self.pwm_num.to_string().as_bytes())
      .chain_err(|| {
        format!(
          "Failed to unexport PWM #{}-{}",
          &self.pwm_chip_num,
          &self.pwm_num
        )
      })?;
    }
    Ok(())
  }

  /// Sets the period of the PWM in nanoseconds.
  ///
  /// # Examples
  ///
  /// ```no_run
  /// use libbeaglebone::prelude::*;
  ///
  /// // Create a new PWM device using PWM chip 0 and PWM 0.
  /// let mut pwm = PWM::new(0, 0);
  ///
  /// // Export the PWM.
  /// pwm.set_export(DeviceState::Exported).unwrap();
  ///
  /// // Make the period 500,000 ns.
  /// pwm.set_period(500_000).unwrap();
  /// ```
  ///
  /// # Errors
  ///
  /// Fails if the pin isn't configured correctly.
  pub fn set_period(&mut self, period_ns: u32) -> Result<()> {
    let path = format!(
      "/sys/class/pwm/pwmchip{}/pwm{}/period",
      &self.pwm_chip_num,
      &self.pwm_num
    );
    path.write_file(&format!("{}", period_ns)).chain_err(|| {
      format!(
        "Failed to set PWM #{}-{} period to {}",
        &self.pwm_chip_num,
        &self.pwm_num,
        period_ns
      )
    })?;
    self.period = period_ns;
    Ok(())
  }

  /// Sets the state (enabled or disabled) of the PWM.
  ///
  /// # Examples
  ///
  /// ```no_run
  /// use libbeaglebone::prelude::*;
  ///
  /// // Create a new PWM device using PWM chip 0 and PWM 0.
  /// let mut pwm = PWM::new(0, 0);
  ///
  /// // Export the PWM.
  /// pwm.set_export(DeviceState::Exported).unwrap();
  ///
  /// // Make the period 500,000 ns.
  /// pwm.set_period(500_000).unwrap();
  ///
  /// // Turn the PWM on
  /// pwm.set_state(PWMState::Enabled).unwrap();
  /// ```
  ///
  /// # Errors
  ///
  /// Fails to if the pin isn't configured correctly.
  pub fn set_state(&mut self, state: PWMState) -> Result<()> {
    let path = format!(
      "/sys/class/pwm/pwmchip{}/pwm{}/enable",
      &self.pwm_chip_num,
      &self.pwm_num
    );
    path.write_file(match state {
      PWMState::Enabled => "1",
      PWMState::Disabled => "0",
    })
        .chain_err(|| {
      format!(
        "Failed to set PWM #{}-{} state to {:?}",
        &self.pwm_chip_num,
        &self.pwm_num,
        state
      )
    })?;
    self.state = state;
    Ok(())
  }

  /// Sets the duty cycle of the PWM as a percentage of the period.
  ///
  /// # Examples
  ///
  /// ```no_run
  /// use libbeaglebone::prelude::*;
  ///
  /// // Create a new PWM device using PWM chip 0 and PWM 0.
  /// let mut pwm = PWM::new(0, 0);
  ///
  /// // Export the PWM.
  /// pwm.set_export(DeviceState::Exported).unwrap();
  ///
  /// // Make the period 500,000 ns.
  /// pwm.set_period(500_000).unwrap();
  ///
  /// // Turn the PWM on.
  /// pwm.set_state(PWMState::Enabled).unwrap();
  ///
  /// // Set the duty cycle to 50% (250,000 ns).
  /// pwm.write(50.0).unwrap();
  /// ```
  ///
  /// # Errors
  ///
  /// Fails if the percentage is less than 0 or exceeds 100, i.e. if the duty
  /// cycle isn't in the period.
  /// Fails to if the pin isn't configured correctly.
  pub fn write(&mut self, percentage: f32) -> Result<()> {
    let path = format!(
      "/sys/class/pwm/pwmchip{}/pwm{}/duty_cycle",
      &self.pwm_chip_num,
      &self.pwm_num
    );
    let new_duty_cycle = ((percentage / 100.0) * (self.period as f32)) as u32;
    path.write_file(&format!("{}", new_duty_cycle)).chain_err(
      || {
        format!(
          "Failed to set PWM #{}-{} duty cycle to {}% (aka {}ns)",
          &self.pwm_chip_num,
          &self.pwm_num,
          percentage,
          new_duty_cycle
        )
      },
    )?;
    self.duty_cycle = new_duty_cycle;
    Ok(())
  }

  /// Sets the duty cycle of the PWM in nanoseconds.
  ///
  /// # Examples
  ///
  /// ```no_run
  /// use libbeaglebone::prelude::*;
  ///
  /// // Create a new PWM device using PWM chip 0 and PWM 0.
  /// let mut pwm = PWM::new(0, 0);
  ///
  /// // Export the PWM.
  /// pwm.set_export(DeviceState::Exported).unwrap();
  ///
  /// // Make the period 500,000 ns.
  /// pwm.set_period(500_000).unwrap();
  ///
  /// // Turn the PWM on.
  /// pwm.set_state(PWMState::Enabled).unwrap();
  ///
  /// // Set the duty cycle to 250,000 ns.
  /// pwm.set_duty_cycle(250_000).unwrap();
  /// ```
  ///
  /// # Errors
  ///
  /// Fails if the duty cycle exceeds the period.
  /// Fails if the pin isn't configured correctly.
  pub fn set_duty_cycle(&mut self, duty_cycle_ns: u32) -> Result<()> {
    let path = format!(
      "/sys/class/pwm/pwmchip{}/pwm{}/duty_cycle",
      &self.pwm_chip_num,
      &self.pwm_num
    );
    path.write_file(&format!("{}", duty_cycle_ns)).chain_err(
      || {
        format!(
          "Failed to set PWM #{}-{} duty cycle to {}ns",
          &self.pwm_chip_num,
          &self.pwm_num,
          duty_cycle_ns
        )
      },
    )?;
    self.duty_cycle = duty_cycle_ns;
    Ok(())
  }
}
