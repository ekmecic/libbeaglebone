use errors::*;
use std::fs::{File, OpenOptions};
use std::marker::PhantomData;
use std::os::unix::io::AsRawFd;

// Constants extracted from linux/spi/spidev.h
bitflags! {
    pub struct SPIModeFlags: u32 {
        /// Clock Phase
        const SPI_CPHA = 0x01;
        /// Clock Polarity
        const SPI_CPOL = 0x02;
        /// Chipselect Active High?
        const SPI_CS_HIGH = 0x04;
        /// Per-word Bits On Wire
        const SPI_LSB_FIRST = 0x08;
        /// SI/SO Signals Shared
        const SPI_3WIRE = 0x10;
        /// Loopback Mode
        const SPI_LOOP = 0x20;
        /// 1 dev/bus; no chipselect
        const SPI_NO_CS = 0x40;
        /// Slave pulls low to pause
        const SPI_READY = 0x80;

        // Common Configurations
        const SPI_MODE_0 = 0x00;
        const SPI_MODE_1 = SPI_CPHA.bits;
        const SPI_MODE_2 = SPI_CPOL.bits;
        const SPI_MODE_3 = (SPI_CPOL.bits | SPI_CPHA.bits);

        /// Transmit with 2 wires
        const SPI_TX_DUAL = 0x100;
        /// Transmit with 4 wires
        const SPI_TX_QUAD = 0x200;
        /// Receive with 2 wires
        const SPI_RX_DUAL = 0x400;
        /// Receive with 4 wires
        const SPI_RX_QUAD = 0x800;
    }
}

#[derive(Debug, Default)]
#[repr(C)]
pub struct spi_ioc_transfer<'a, 'b> {
  tx_buf: u64,
  rx_buf: u64,
  len: u32,

  tx_buf_ref: PhantomData<&'a [u8]>,
  rx_buf_ref: PhantomData<&'b mut [u8]>,
}

impl<'a, 'b> spi_ioc_transfer<'a, 'b> {
  pub fn read(buf: &'b mut [u8]) -> Self {
    spi_ioc_transfer {
      rx_buf: buf.as_ptr() as *const () as usize as u64,
      len: buf.len() as u32,
      ..Default::default()
    }
  }

  pub fn write(buf: &'a [u8]) -> Self {
    spi_ioc_transfer {
      tx_buf: buf.as_ptr() as *const () as usize as u64,
      len: buf.len() as u32,
      ..Default::default()
    }
  }

  /// The `tx_buf` and `rx_buf` must be the same length.
  pub fn read_write(tx_buf: &'a [u8], rx_buf: &'b mut [u8]) -> Self {
    assert_eq!(tx_buf.len(), rx_buf.len());
    spi_ioc_transfer {
      rx_buf: rx_buf.as_ptr() as *const () as usize as u64,
      tx_buf: tx_buf.as_ptr() as *const () as usize as u64,
      len: tx_buf.len() as u32,
      ..Default::default()
    }
  }
}

pub type SpidevTransfer<'a, 'b> = spi_ioc_transfer<'a, 'b>;

// IOCTL magic numbers
const SPI_IOC_MAGIC: u8 = b'k';
const SPI_IOC_NR_TRANSFER: u8 = 0;
const SPI_IOC_NR_MODE: u8 = 1;
const SPI_IOC_NR_LSB_FIRST: u8 = 2;
const SPI_IOC_NR_BITS_PER_WORD: u8 = 3;
const SPI_IOC_NR_MAX_SPEED_HZ: u8 = 4;
const SPI_IOC_NR_MODE32: u8 = 5;

// IOCTL functions: these macros expand to safe-ish wrappers for IOCTL, which
// are then called by the accessors and mutators below
ioctl!(read get_mode_u8 with SPI_IOC_MAGIC, SPI_IOC_NR_MODE; u8);
ioctl!(read get_mode_u32 with SPI_IOC_MAGIC, SPI_IOC_NR_MODE; u32);
ioctl!(write set_mode_u8 with SPI_IOC_MAGIC, SPI_IOC_NR_MODE; u8);
ioctl!(write set_mode_u32 with SPI_IOC_MAGIC, SPI_IOC_NR_MODE32; u32);
ioctl!(read  get_lsb_first with SPI_IOC_MAGIC, SPI_IOC_NR_LSB_FIRST; u8);
ioctl!(write set_lsb_first with SPI_IOC_MAGIC, SPI_IOC_NR_LSB_FIRST; u8);
ioctl!(read  get_bits_per_word with SPI_IOC_MAGIC, SPI_IOC_NR_BITS_PER_WORD; u8);
ioctl!(write set_bits_per_word with SPI_IOC_MAGIC, SPI_IOC_NR_BITS_PER_WORD; u8);
ioctl!(read  get_max_speed_hz with SPI_IOC_MAGIC, SPI_IOC_NR_MAX_SPEED_HZ; u32);
ioctl!(write set_max_speed_hz with SPI_IOC_MAGIC, SPI_IOC_NR_MAX_SPEED_HZ; u32);
ioctl!(write spidev_transfer with SPI_IOC_MAGIC, SPI_IOC_NR_TRANSFER; spi_ioc_transfer);

/// Represents a SPI interface.
#[derive(Debug)]
pub struct SPI {
  bits_per_word: u8,
  max_speed_hz: u32,
  lsb_first: bool,
  spi_mode: SPIModeFlags,
  spi_file: File,
}

impl SPI {
  pub fn new(spi_num: u8) -> Result<SPI> {
    let spi_file_path = format!("/dev/spidev{}.0", spi_num);
    Ok(SPI {
         bits_per_word: 8,
         max_speed_hz: 10_000,
         lsb_first: false,
         spi_mode: SPI_MODE_0,
         spi_file: OpenOptions::new()
           .read(true)
           .write(true)
           .open(spi_file_path)
           .chain_err(|| format!("Failed to create new SPI device #{}.", spi_num))?,
       })
  }

  pub fn get_mode(&self) -> Result<u8> {
    let mut mode: u8 = 0;
    unsafe {
      let _ = get_mode_u8(self.spi_file.as_raw_fd(), &mut mode)
        .chain_err(|| "Failed to read SPI mode.")?;
    };
    Ok(mode)
  }

  pub fn set_mode(&self, mode: SPIModeFlags) -> Result<()> {
    if (mode.bits & 0xFFFFFF00) != 0 {
      unsafe {
        let _ = set_mode_u32(self.spi_file.as_raw_fd(), &mode.bits)
          .chain_err(|| "Failed to set SPI mode.")?;
      };
    } else {
      let bits: u8 = mode.bits as u8;
      unsafe {
        let _ = set_mode_u8(self.spi_file.as_raw_fd(), &bits)
          .chain_err(|| "Failed to set SPI mode.")?;
      };
    }
    Ok(())
  }

  pub fn get_lsb_first(&self) -> Result<u8> {
    let mut lsb_first: u8 = 0;
    unsafe {
      let _ = get_lsb_first(self.spi_file.as_raw_fd(), &mut lsb_first)
        .chain_err(|| "Failed to read SPI LSB setting.")?;
    };
    Ok(lsb_first)
  }

  pub fn set_lsb_first(&self, lsb_first: bool) -> Result<()> {
    let lsb_first_value: u8 = if lsb_first { 1 } else { 0 };
    unsafe {
      let _ = set_lsb_first(self.spi_file.as_raw_fd(), &lsb_first_value)
        .chain_err(|| "Failed to set SPI LSB setting.")?;
    };
    Ok(())
  }

  pub fn get_bits_per_word(&self) -> Result<u8> {
    let mut bits_per_word: u8 = 0;
    unsafe {
      let _ = get_bits_per_word(self.spi_file.as_raw_fd(), &mut bits_per_word)
        .chain_err(|| "Failed to read SPI bits per word.")?;
    };
    Ok(bits_per_word)
  }

  pub fn set_bits_per_word(&self, bits_per_word: u8) -> Result<()> {
    unsafe {
      let _ = set_bits_per_word(self.spi_file.as_raw_fd(), &bits_per_word)
        .chain_err(|| "Failed to set SPI bits per word.")?;
    };
    Ok(())
  }

  pub fn get_max_speed_hz(&self) -> Result<u32> {
    let mut max_speed_hz: u32 = 0;
    unsafe {
      let _ = get_max_speed_hz(self.spi_file.as_raw_fd(), &mut max_speed_hz)
        .chain_err(|| "Failed to read SPI speed.")?;
    };
    Ok(max_speed_hz)
  }

  pub fn set_max_speed_hz(&self, max_speed_hz: u32) -> Result<()> {
    unsafe {
      let _ = set_max_speed_hz(self.spi_file.as_raw_fd(), &max_speed_hz)
        .chain_err(|| "Failed to set SPI speed.")?;
    };
    Ok(())
  }
  pub fn transfer(&self, transfer: &mut SpidevTransfer) -> Result<()> {
    // The kernel will directly modify the rx_buf of the SpidevTransfer
    // rx_buf if present, so there is no need to do any additional work
    unsafe {
      let _ = spidev_transfer(self.spi_file.as_raw_fd(), transfer)
        .chain_err(|| "failed to transfer over SPI")?;
    };
    Ok(())
  }
}
