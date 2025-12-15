use crate::Error;
use bitfield_struct::bitfield;
use embedded_hal::i2c::I2c;

pub struct Slg46824x<I2C> {
    i2c: I2C,
    slave_select: u8,
}

impl<I2C: I2c> Slg46824x<I2C> {
    /// Create new Slg46824x instance for communicating with a slave with `slave_select` control code.
    /// There can be up to 16 devices on the same bus, so `slave_select` must be within 0..=15 range (configured in SLG bitstream).
    pub fn new(i2c: I2C, slave_select: u8) -> Option<Self> {
        if slave_select > 15 {
            return None;
        }
        Some(Self { i2c, slave_select })
    }

    /// Read up to 256 bytes (2048 bits) from RAM.
    /// Returns an [Error::OutOfBounds] if (data.len() + word_addr > 256).
    pub fn read_ram(&mut self, word_addr: u8, data: &mut [u8]) -> Result<(), Error<I2C::Error>> {
        if word_addr as usize + data.len() > 256 {
            return Err(Error::OutOfBounds);
        }
        self.i2c
            .write_read(self.slave_addr_ram(), &[word_addr], data)?;
        Ok(())
    }

    /// Read up to 256 bytes (2048 bits) from NVM.
    /// Returns an [Error::OutOfBounds] if (data.len() + word_addr > 256).
    pub fn read_nvm(&mut self, word_addr: u8, data: &mut [u8]) -> Result<(), Error<I2C::Error>> {
        if word_addr as usize + data.len() > 256 {
            return Err(Error::OutOfBounds);
        }
        self.i2c
            .write_read(self.slave_addr_nvm(), &[word_addr], data)?;
        Ok(())
    }

    fn slave_addr_ram(&self) -> u8 {
        I2cAddress::new()
            .with_control_code(self.slave_select)
            .with_a10(false)
            .with_a9(false)
            .with_a8(false)
            .into_bits()
    }

    fn slave_addr_nvm(&self) -> u8 {
        I2cAddress::new()
            .with_control_code(self.slave_select)
            .with_a10(false)
            .with_a9(true)
            .with_a8(false)
            .into_bits()
    }
}

#[bitfield(u8, order = Msb)]
struct I2cAddress {
    #[bits(1)]
    _reserved: bool,
    #[bits(4)]
    control_code: u8,
    #[bits(1)]
    a10: bool,
    #[bits(1)]
    a9: bool,
    #[bits(1)]
    a8: bool,
}
