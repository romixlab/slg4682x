use crate::Error;
use crate::i2c_comm::Slg46824x;
use crate::matrix_input::MatrixInput;
use crate::matrix_output::MatrixOutput;
use embedded_hal::i2c::I2c;

impl<I2C: I2c> Slg46824x<I2C> {
    /// Wire the matrix connection from input to output.
    /// An input signal can be routed to one or more outputs at the same time.
    pub fn matrix_wire(
        &mut self,
        to: MatrixOutput,
        from: MatrixInput,
    ) -> Result<(), Error<I2C::Error>> {
        self.write_ram_masked(to.value(), from.value())?;
        Ok(())
    }
}
