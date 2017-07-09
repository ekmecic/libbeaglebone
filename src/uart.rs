//! The UART module.

use errors::*;
use serialport::open;
use serialport::prelude::*;
use std::time::Duration;

/// The direction of the pin, which can be either an input or output.
#[allow(missing_debug_implementations)]
pub struct UART {
  port: Box<SerialPort>,
}

impl UART {
  /// Creates a new UART port.
  pub fn new(uart_num: u32) -> Result<(UART)> {
    let port_path = format!("/dev/ttyO{}", uart_num);
    Ok(UART {
         port: open(&port_path)
           .chain_err(|| "Unable to open UART port")?,
       })
  }

  /// Write data to a UART port.
  pub fn write(&mut self, data: &str) -> Result<()> {
    self.port
        .write_all(data.as_bytes())
        .chain_err(|| "Failed to write to UART port")?;
    Ok(())
  }

  /// Read the specified number of bytes from the UART port.
  pub fn read_chars(&mut self, num_bytes: usize) -> Result<(Vec<u8>)> {
    let mut buf: Vec<u8> = Vec::with_capacity(num_bytes);

    self.port
        .read_exact(buf.as_mut_slice())
        .chain_err(|| "Failed to read from to UART port")?;

    Ok(buf)
  }

  /// Read the specified number of bytes and return it as a string.
  pub fn read_to_string(&mut self, num_bytes: usize) -> Result<(String)> {
    let mut buf: Vec<u8> = Vec::with_capacity(num_bytes);

    self.port
        .read_exact(buf.as_mut_slice())
        .chain_err(|| "Failed to read from to UART port")?;

    Ok(String::from_utf8(buf)
         .chain_err(|| "Failed to convert the bytes from the UART port to a string.")?)
  }


  /// Get the all of the UART port settings.
  pub fn settings(&self) -> SerialPortSettings {
    self.port.settings()
  }

  /// Get the baud rate setting of the UART port.
  pub fn baud_rate(&self) -> Option<BaudRate> {
    self.port.baud_rate()
  }

  /// Get the data bits setting of the UART port.
  pub fn data_bits(&self) -> Option<DataBits> {
    self.port.data_bits()
  }

  /// Get the flow control setting of the UART port.
  pub fn flow_control(&self) -> Option<FlowControl> {
    self.port.flow_control()
  }

  /// Get the parity setting of the UART port.
  pub fn parity(&self) -> Option<Parity> {
    self.port.parity()
  }

  /// Get the stop bits setting of the UART port.
  pub fn stop_bits(&self) -> Option<StopBits> {
    self.port.stop_bits()
  }

  /// Get the timeout setting of the UART port.
  pub fn timeout(&self) -> Duration {
    self.port.timeout()
  }

  /// Set the baud rate on the UART port.
  pub fn set_baud_rate(&mut self, baud_rate: BaudRate) -> Result<()> {
    Ok(self.port
           .set_baud_rate(baud_rate)
           .chain_err(|| "Unable to set UART baud rate.")?)
  }

  /// Set the data bits on the UART port.
  pub fn set_data_bits(&mut self, data_bits: DataBits) -> Result<()> {
    Ok(self.port
           .set_data_bits(data_bits)
           .chain_err(|| "Unable to set UART data bits.")?)
  }

  /// Set the flow control on the UART port.
  pub fn set_flow_control(&mut self, flow_control: FlowControl) -> Result<()> {
    Ok(self.port
           .set_flow_control(flow_control)
           .chain_err(|| "Unable to set UART flow control.")?)
  }

  /// Set the parity on the UART port.
  pub fn set_parity(&mut self, parity: Parity) -> Result<()> {
    Ok(self.port
           .set_parity(parity)
           .chain_err(|| "Unable to set UART parity.")?)
  }

  /// Set the stop bits on the UART port.
  pub fn set_stop_bits(&mut self, stop_bits: StopBits) -> Result<()> {
    Ok(self.port
           .set_stop_bits(stop_bits)
           .chain_err(|| "Unable to set UART stop bits.")?)
  }

  /// Set the timeout on the UART port.
  pub fn set_timeout(&mut self, timeout: Duration) -> Result<()> {
    Ok(self.port
           .set_timeout(timeout)
           .chain_err(|| "Unable to set UART timeout.")?)
  }
}
