//! The I2C module.

use errors::*;
use std::fs::{File, OpenOptions};
use std::mem;
use std::os::unix::io::AsRawFd;

/// Magic I2C numbers
const I2C_SMBUS_BLOCK_MAX: u8 = 32;
const I2C_SLAVE: u16 = 0x0703;
const I2C_SMBUS: u16 = 0x0720;

/// Where we store data that's read off an I2C device
// TODO: switch to an untagged union once that feature is released.
// GitHub issue: https://github.com/rust-lang/rust/issues/32836.
#[repr(C)]
struct i2c_data {
  block: [u8; (I2C_SMBUS_BLOCK_MAX + 2) as usize],
}

/// The structure used by i2c_call to interact with the i2cdev system.
#[repr(C)]
struct i2c_ioctl_data {
  read_write: u8, // __u8 read_write;
  command: u8, // __u8 command;
  size: u32, // __u32 size;
  data: *mut i2c_data, // union i2c_smbus_data __user *data;
}

// These macros expand to the nice IOCTL wrapper functions needed to work with
// the i2cdev system.
ioctl!(ioctl_set_i2c_slave_addr with I2C_SLAVE);
ioctl!(ioctl_i2c_smbus with I2C_SMBUS);

/// I2C device
#[derive(Debug)]
pub struct I2C {
  i2c_num: u8,
  i2c_file: File,
}

impl I2C {
  /// Creates a new I2C device.
  pub fn new(m_i2c_num: u8) -> Result<(I2C)> {
    let i2c_file_path = format!("/dev/i2c-{}", m_i2c_num);
    Ok(I2C {
         i2c_num: m_i2c_num,
         i2c_file: OpenOptions::new()
           .read(true)
           .write(true)
           .open(i2c_file_path)
           .chain_err(|| "Unable to create new I2C device.")?,
       })
  }

  /// Writes a single byte to an I2C slave.
  pub fn write(&self, data: u8) -> Result<(i32)> {
    unsafe {
      Ok(self.i2c_call(0 /* 0 => write */, data, 1, ::std::ptr::null_mut())
             .chain_err(|| "ll")?)
    }
  }

  /// Reads a single byte from an I2C slave and returns it.
  pub fn read(&mut self) -> Result<(u8)> {
    let mut data = i2c_data { block: unsafe { mem::zeroed() } };
    unsafe {
      // TODO: fix this ugly temporary hack to avoid the "unused result" warning
      let _ = self.i2c_call(1, 0, 1, &mut data).chain_err(|| "")?;
    };
    Ok(data.block[0])
  }

  /// sets the addr of the slave
  pub fn set_slave_address(&self, slave_addr: u16) -> Result<(i32)> {
    unsafe {
      Ok(ioctl_set_i2c_slave_addr(self.i2c_file.as_raw_fd(), slave_addr as *mut u8)
           .chain_err(|| "unable to set slave device address.")?)
    }
  }

  /// Wrapper function around the IOCTL FFI that does all of the IOCTL dirty
  /// work.
  unsafe fn i2c_call(&self,
                     read_write: u8,
                     command: u8, // can be address or something else
                     size: u32,
                     data: *mut i2c_data)
                     -> Result<(i32)> {

    let mut args = i2c_ioctl_data {
      read_write: read_write,
      command: command,
      size: size,
      data: data,
    };

    // Transmuting is ugly, but it's needed to remove all of the type information
    // that Rust keeps in the struct.
    let p_args: *mut u8 = mem::transmute(&mut args);
    Ok(ioctl_i2c_smbus(self.i2c_file.as_raw_fd(), p_args)
         .chain_err(|| "failed to write to i2c bus")?)
  }
}
