//! The GPIO module
use util::*;
use std::path::PathBuf;

/// Represents a pin configured as a GPIO
#[derive(Debug)]
pub struct GPIO {
    pin_num: u8,
    pin_path: PathBuf,
}

impl GPIO {
    /// Creates a new GPIO pin
    pub fn new(m_pin_num: u8) -> GPIO {
        let m_pin_path = format!("/sys/class/gpio/gpio{}", m_pin_num);
        GPIO {
            pin_num: m_pin_num,
            pin_path: PathBuf::from(m_pin_path),
        }
    }
    /// Export a GPIO pin
    pub fn export(&self) {
        // If the path exists, don't do anything
        // If it doesn't exist, create it and export
        if !self.pin_path.exists() {
            write_to_file(&self.pin_num.to_string(), "/sys/class/gpio/export")
        }
    }
    /// Unexport a GPIO pin
    pub fn unexport(&self) {
        // If the path exists, try to unexport it
        // If it doesn't exist, don't do anything
        if !self.pin_path.exists() {
            write_to_file(&self.pin_num.to_string(), "/sys/class/gpio/export")
        }
    }
    /// Set the state hi or lo
    pub fn set_state(&self, state: bool) {
        // TODO: replace with something less sketchy
        if state {
            write_to_file("1", &format!("/sys/class/gpio/gpio{}/value", self.pin_num))
        } else {
            write_to_file("0", &format!("/sys/class/gpio/gpio{}/value", self.pin_num))
        }
    }
}
