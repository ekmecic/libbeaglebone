//! The GPIO module

/// Represents a pin configured as a GPIO
#[derive(Debug)]
pub struct GPIO {
    pin_num: u8,
}

impl GPIO {
    /// Creates a new GPIO pin
    pub fn new(m_pin_num: u8) -> GPIO {
        GPIO { pin_num: m_pin_num }
    }
}
