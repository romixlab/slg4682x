#![cfg_attr(not(feature = "std"), no_std)]

use core::fmt::{Debug, Display, Formatter};

mod hal;
pub mod i2c_comm;
pub mod matrix_input;
pub mod matrix_output;

#[derive(Debug)]
pub enum Error<E> {
    OutOfBounds,
    I2cError(E),
}

impl<E: Debug> Display for Error<E> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:02X?}", self)
    }
}

impl<E> From<E> for Error<E> {
    fn from(e: E) -> Self {
        Self::I2cError(e)
    }
}

#[cfg(feature = "std")]
impl<E: Debug> std::error::Error for Error<E> {}
