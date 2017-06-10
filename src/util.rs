//! general utils

use std::fs::File;
use std::io::{Write, Read};
use errors::*;

/// Write some data to a file
pub fn write_file(data: &str, attr: &str, pin_num: &u8) -> Result<()> {
    let path = format!("/sys/class/gpio/gpio{}/{}", pin_num, attr);

    // Open the file (write-only) and write data to it
    File::create(&path)
        .chain_err(|| format!("Failed to open file {} for writing", &path))?
    .write_all(data.as_bytes())
        .chain_err(|| format!("Failed to write to file {}", &path))?;
    Ok(())
}

/// read from a device file
pub fn read_file(attr: &str, pin_num: &u8) -> Result<(bool)> {
    let path = format!("/sys/class/gpio/gpio{}/{}", pin_num, attr);
    let mut value_str = String::new();

    // Open the file (read-only) and read it's contents into the string
    let _ = File::open(&path)
        .chain_err(|| format!("Failed to open file {} for reading", &path))?
    .read_to_string(&mut value_str)
        .chain_err(|| format!("Failed to read from file {}", &path))?;

    // Convert the string into a bool
    Ok(match value_str.trim() {
        "0" => false,
        "1" => true,
        _ => bail!(format!("Invalid value read from file {}", &path)),
    })
}
