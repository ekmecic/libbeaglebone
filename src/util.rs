//! General utility functions used internally throughout the crate, such as
//! writing to sysfs files.

use errors::*;
use std::fs::File;
use std::io::{Write, Read};

/// Writes data to a sysfs device file.
pub fn write_file(data: &str, path: &str) -> Result<()> {
  // Open the file (write-only) and write data to it
  File::create(&path)
    .chain_err(|| format!("Failed to open file {} for writing", &path))?
    .write_all(data.as_bytes())
    .chain_err(|| format!("Failed to write to file {}", &path))?;
  Ok(())
}

/// Reads from a sysfs device file.
pub fn read_file(path: &str) -> Result<String> {
  let mut value_str = String::new();

  // Open the file (read-only) and read it's contents into the string
  let _ = File::open(path)
    .chain_err(|| format!("Failed to open file {} for reading", &path))?
    .read_to_string(&mut value_str)
    .chain_err(|| format!("Failed to read from file {}", &path))?;

  Ok(value_str)
}
