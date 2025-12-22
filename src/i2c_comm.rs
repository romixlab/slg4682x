use crate::Error;
use bitfield_struct::bitfield;
use embedded_hal::i2c::I2c;
use std::ops::RangeInclusive;

/// See p.113 of the datasheet
const MASK_REG: u8 = 0xC9;

pub struct Slg46824x<I2C> {
    i2c: I2C,
    slave_select: u8,
}

impl<I2C: I2c> Slg46824x<I2C> {
    /// Create new Slg46824x instance for communicating with a slave with `slave_select` control code.
    /// There can be up to 14 devices on the same bus (+ 2 I2C broadcast addresses), so `slave_select` must be within 0..=15 range (configured in SLG bitstream).
    /// Default slave_select value is 1.
    pub fn new(i2c: I2C, slave_select: u8) -> Option<Self> {
        if slave_select > 15 {
            return None;
        }
        Some(Self { i2c, slave_select })
    }

    /// Create new Slg46824x instance with default `slave_select` = 1.
    pub fn new_default(i2c: I2C) -> Self {
        Self {
            i2c,
            slave_select: 1,
        }
    }

    /// Read up to 256 bytes (2048 bits) from RAM.
    /// Returns an [Error::OutOfBounds] if (data.len() + word_addr > 256).
    pub fn read_ram(&mut self, word_addr: u8, data: &mut [u8]) -> Result<(), Error<I2C::Error>> {
        if word_addr as usize + data.len() > 256 {
            return Err(Error::OutOfBounds);
        }
        self.i2c.write_read(self.ram_addr(), &[word_addr], data)?;
        Ok(())
    }

    /// Read up to 256 bytes (2048 bits) from NVM.
    /// Returns an [Error::OutOfBounds] if (data.len() + word_addr > 256).
    pub fn read_nvm(&mut self, word_addr: u8, data: &mut [u8]) -> Result<(), Error<I2C::Error>> {
        if word_addr as usize + data.len() > 256 {
            return Err(Error::OutOfBounds);
        }
        self.i2c.write_read(self.nvm_addr(), &[word_addr], data)?;
        Ok(())
    }

    pub fn write_ram_masked(
        &mut self,
        bit_range: RangeInclusive<usize>,
        value: u8,
    ) -> Result<(), Error<I2C::Error>> {
        let msb_global = *bit_range.start();
        let lsb_global = *bit_range.end();
        debug_assert!(msb_global > lsb_global);
        debug_assert!(msb_global - lsb_global + 1 <= 8);

        let hi_byte = (msb_global / 8) as u8;
        let msb = msb_global % 8; // 0..=7, msb:0 in hi byte
        let lo_byte = (lsb_global / 8) as u8;
        let lsb = lsb_global % 8; // 0..=7, 7:lsb in lo byte
        if hi_byte == lo_byte {
            if msb == 7 && lsb == 0 {
                // 1 normal write
                self.i2c.write(self.ram_addr(), &[hi_byte, value])?;
            } else {
                // 1 masked write
                let n_mask = !((0xFFu8 << lsb) & (0xFFu8 >> (7 - msb))); // 1: keep existing bit, 0: overwrite
                self.i2c.write(self.ram_addr(), &[MASK_REG, n_mask])?;
                self.i2c.write(self.ram_addr(), &[hi_byte, value])?;
            }
        } else {
            // 2 masked writes
            let hi_n_mask = 0xFFu8 << (msb + 1);
            let lo_n_mask = 0xFFu8 >> (8 - lsb);
            let hi_value = value >> (8 - lsb);
            let lo_value = value << lsb;
            self.i2c.write(self.ram_addr(), &[MASK_REG, hi_n_mask])?;
            self.i2c.write(self.ram_addr(), &[hi_byte, hi_value])?;
            self.i2c.write(self.ram_addr(), &[MASK_REG, lo_n_mask])?;
            self.i2c.write(self.ram_addr(), &[lo_byte, lo_value])?;
        }
        Ok(())
    }

    pub fn read_ram_masked(
        &mut self,
        bit_range: RangeInclusive<usize>,
    ) -> Result<u8, Error<I2C::Error>> {
        self.read_masked(bit_range, self.ram_addr())
    }

    pub fn read_nvm_masked(
        &mut self,
        bit_range: RangeInclusive<usize>,
    ) -> Result<u8, Error<I2C::Error>> {
        self.read_masked(bit_range, self.nvm_addr())
    }

    fn read_masked(
        &mut self,
        bit_range: RangeInclusive<usize>,
        addr: u8,
    ) -> Result<u8, Error<I2C::Error>> {
        let msb_global = *bit_range.start();
        let lsb_global = *bit_range.end();
        debug_assert!(msb_global > lsb_global);
        debug_assert!(msb_global - lsb_global + 1 <= 8);

        let hi_byte = (msb_global / 8) as u8;
        let msb = msb_global % 8; // 0..=7, msb:0 in hi byte
        let lo_byte = (lsb_global / 8) as u8;
        let lsb = lsb_global % 8; // 0..=7, 7:lsb in lo byte
        if hi_byte == lo_byte {
            let mut value = [0u8; 1];
            self.i2c.write_read(addr, &[lo_byte], &mut value)?;
            let value = value[0] >> lsb;
            Ok(value & (0xFF >> (7 - msb)))
        } else {
            let mut value = [0u8; 2];
            self.i2c.write_read(addr, &[lo_byte], &mut value)?;
            let hi_value = value[1];
            let lo_value = value[0];
            Ok((hi_value << (7 - msb)) | (lo_value >> lsb))
        }
    }

    fn ram_addr(&self) -> u8 {
        I2cAddress::new()
            .with_control_code(self.slave_select)
            .with_a10(false)
            .with_a9(false)
            .with_a8(false)
            .into_bits()
    }

    fn nvm_addr(&self) -> u8 {
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

#[cfg(test)]
mod tests {
    use crate::i2c_comm::{MASK_REG, Slg46824x};
    use embedded_hal::i2c::{ErrorType, I2c, Operation, SevenBitAddress};
    use std::collections::VecDeque;

    #[derive(Default)]
    struct I2cMock {
        writes: VecDeque<Vec<u8>>,
    }

    impl I2c for I2cMock {
        fn transaction(
            &mut self,
            _address: SevenBitAddress,
            operations: &mut [Operation<'_>],
        ) -> Result<(), Self::Error> {
            for op in operations {
                match op {
                    Operation::Read(_) => {}
                    Operation::Write(data) => self.writes.push_back(data.to_vec()),
                }
            }
            Ok(())
        }
    }

    impl I2cMock {
        fn pop_front(&mut self) -> Vec<u8> {
            self.writes.pop_front().unwrap()
        }
    }

    #[derive(Debug)]
    struct Error;

    impl ErrorType for I2cMock {
        type Error = Error;
    }

    impl embedded_hal::i2c::Error for Error {
        fn kind(&self) -> embedded_hal::i2c::ErrorKind {
            match self {
                _ => embedded_hal::i2c::ErrorKind::Other,
            }
        }
    }

    #[test]
    fn write_masked() {
        let mut slg = Slg46824x::new_default(I2cMock::default());

        slg.write_ram_masked(7..=0, 0xAA).unwrap();
        assert_eq!(slg.i2c.pop_front().as_slice(), &[0, 0xAA]);
        assert!(slg.i2c.writes.is_empty());

        slg.write_ram_masked(15..=8, 0xAA).unwrap();
        assert_eq!(slg.i2c.pop_front().as_slice(), &[1, 0xAA]);
        assert!(slg.i2c.writes.is_empty());

        slg.write_ram_masked(6..=1, 0xAA).unwrap();
        assert_eq!(slg.i2c.pop_front().as_slice(), &[MASK_REG, 0b1000_0001]);
        assert_eq!(slg.i2c.pop_front().as_slice(), &[0, 0xAA]);
        assert!(slg.i2c.writes.is_empty());

        slg.write_ram_masked(14..=9, 0xAA).unwrap();
        assert_eq!(slg.i2c.pop_front().as_slice(), &[MASK_REG, 0b1000_0001]);
        assert_eq!(slg.i2c.pop_front().as_slice(), &[1, 0xAA]);
        assert!(slg.i2c.writes.is_empty());

        slg.write_ram_masked(11..=6, 0b0010_0101).unwrap();
        assert_eq!(slg.i2c.pop_front().as_slice(), &[MASK_REG, 0b1111_0000]);
        assert_eq!(slg.i2c.pop_front().as_slice(), &[1, 0b0000_1001]);
        assert_eq!(slg.i2c.pop_front().as_slice(), &[MASK_REG, 0b0011_1111]);
        assert_eq!(slg.i2c.pop_front().as_slice(), &[0, 0b0100_0000]);
        assert!(slg.i2c.writes.is_empty());

        slg.write_ram_masked(8..=1, 0b1010_0101).unwrap();
        assert_eq!(slg.i2c.pop_front().as_slice(), &[MASK_REG, 0b1111_1110]);
        assert_eq!(slg.i2c.pop_front().as_slice(), &[1, 0b0000_0001]);
        assert_eq!(slg.i2c.pop_front().as_slice(), &[MASK_REG, 0b0000_0001]);
        assert_eq!(slg.i2c.pop_front().as_slice(), &[0, 0b0100_1010]);
        assert!(slg.i2c.writes.is_empty());

        slg.write_ram_masked(14..=7, 0b1010_0101).unwrap();
        assert_eq!(slg.i2c.pop_front().as_slice(), &[MASK_REG, 0b1000_0000]);
        assert_eq!(slg.i2c.pop_front().as_slice(), &[1, 0b0101_0010]);
        assert_eq!(slg.i2c.pop_front().as_slice(), &[MASK_REG, 0b0111_1111]);
        assert_eq!(slg.i2c.pop_front().as_slice(), &[0, 0b1000_0000]);
        assert!(slg.i2c.writes.is_empty());
    }
}
