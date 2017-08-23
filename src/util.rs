//! General utility functions used internally throughout the crate, such as
//! writing to sysfs files.

use errors::*;
use std::fs::File;
use std::io::{Write, Read};

pub trait Writeable {
  fn write_file(self, data: &str) -> Result<()>;
}

pub trait Readable {
  fn read_file(self) -> Result<String>;
}

impl<'a> Writeable for &'a str {
  /// Writes data to a sysfs device file.
  fn write_file(self, data: &str) -> Result<()> {
    // Open the file (write-only) and write data to it
    File::create(self)
      .chain_err(|| format!("Failed to open file {} for writing", self))?
      .write_all(data.as_bytes())
      .chain_err(|| format!("Failed to write to file {}", self))?;
    Ok(())
  }
}

impl Writeable for File {
  /// Writes data to a sysfs device file.
  fn write_file(mut self, data: &str) -> Result<()> {
    let _ = self.write_all(data.as_bytes());
    Ok(())
  }
}

impl<'a> Readable for &'a str {
  /// Reads from a sysfs device file.
  fn read_file(self) -> Result<String> {
    let mut value_str = String::new();

    // Open the file (read-only) and read it's contents into the string
    let _ = File::open(self)
      .chain_err(|| format!("Failed to open file {} for reading", self))?
      .read_to_string(&mut value_str)
      .chain_err(|| format!("Failed to read from file {}", self))?;

    Ok(value_str)
  }
}

impl Readable for File {
  /// Reads from a sysfs device file.
  fn read_file(mut self) -> Result<String> {
    let mut value_str = String::new();
    let _ = self.read_to_string(&mut value_str);
    Ok(value_str)
  }
}
