//! The I2C module.
//!
//! The BeagleBone Black has 2 I2C interfaces available (I2C1 and I2C2).
//! These can be enabled by using the bone_capemgr module.
//!
//! As an example, use the following command to enable UART4:
//! `sudo sh -c "echo 'BB-I2C1' > /sys/devices/platform/bone_capemgr/slots"`
//! This command works for recent kernel versions.
//! If you wish to enable another I2C, substitute its number for 1 in the
//! command above.

use errors::*;
use std::fs::{File, OpenOptions};
use std::os::unix::io::AsRawFd;
use util::*;

/// Magic I2C numbers
const I2C_SLAVE: u16 = 0x0703;

// These macros expand to the nice IOCTL wrapper functions needed to work with
// the i2cdev system.
ioctl!(ioctl_set_i2c_slave_addr with I2C_SLAVE);

/// Represents and I2C interface.
#[derive(Debug)]
pub struct I2C {
  i2c_num: u8,
  i2c_file: File,
}

impl I2C {
  /// Creates a new I2C device.
  ///
  /// # Examples
  ///
  /// ```no_run
  /// use libbeaglebone::prelude::*;
  ///
  /// // Create a new I2C interface using BB_I2C1.
  /// // Don't forget to enable the I2C beforehand using bone_capemgr.
  /// // Consult the module documentation for more information!
  /// let i2c = I2C::new(1).unwrap();
  /// ```
  ///
  /// # Errors
  ///
  /// Method fails if `i2c_num` is an invalid I2C port (i.e. isn't within 1-2)
  /// or if the kernel fails to open the port for some other reason.
  pub fn new(i2c_num: u8) -> Result<(I2C)> {
    Ok(I2C {
      i2c_num: i2c_num,
      i2c_file: OpenOptions::new()
        .read(true)
        .write(true)
        .open(&format!("/dev/i2c-{}", i2c_num))
        .chain_err(|| format!("Failed to open new I2C device #{}.", i2c_num))?,
    })
  }

  /// Sets the address of the I2C slave device.
  ///
  /// # Examples
  ///
  /// ```no_run
  /// use libbeaglebone::prelude::*;
  ///
  /// // Create a new I2C interface using BB_I2C1.
  /// let i2c = I2C::new(1).unwrap();
  ///
  /// // Set the slave address to 0x45.
  /// i2c.set_slave_address(0x45).unwrap();
  /// ```
  ///
  /// # Errors
  ///
  /// Fails if the kernel is unable to set the slave device address to the
  /// chosen value.
  pub fn set_slave_address(&self, slave_addr: u16) -> Result<()> {
    unsafe {
      let _ = ioctl_set_i2c_slave_addr(self.i2c_file.as_raw_fd(), slave_addr as *mut u8)
        .chain_err(|| {
          format!("Failed to set I2C slave device address to {}.", slave_addr)
        })?;
      Ok(())
    }
  }

  /// Writes a single byte to an I2C slave.
  ///
  /// # Examples
  ///
  /// ```no_run
  /// use libbeaglebone::prelude::*;
  ///
  /// // Create a new I2C interface using BB_I2C1.
  /// let i2c = I2C::new(1).unwrap();
  ///
  /// // Set the slave address to 0x45.
  /// i2c.set_slave_address(0x45).unwrap();
  ///
  /// // Write a 1 to the I2C slave
  /// i2c.write(1).unwrap();
  /// ```
  ///
  /// # Errors
  ///
  /// Fails if the kernel is unable to write the chosen value to the device.
  pub fn write(self, data: u8) -> Result<()> {
    Ok(self.i2c_file.write_file(&(data.to_string()))?)
  }

  /// Reads a single byte from an I2C slave and returns it.
  ///
  /// # Examples
  ///
  /// ```no_run
  /// use libbeaglebone::prelude::*;
  ///
  /// // Create a new I2C interface using BB_I2C1.
  /// let mut i2c = I2C::new(1).unwrap();
  ///
  /// // Set the slave address to 0x45.
  /// i2c.set_slave_address(0x45).unwrap();
  ///
  /// // Read some data from the I2C device and display it.
  /// println!("Read {} from the I2C slave!", i2c.read().unwrap());
  /// ```
  ///
  /// # Errors
  ///
  /// Fails if the kernel is unable to read from the device.
  pub fn read(self) -> Result<(u8)> {
    let res = self.i2c_file.read_file()?;
    Ok(res.trim().parse::<u8>().unwrap())
  }
}
