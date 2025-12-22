use anyhow::Result;
use slg4682x::i2c_comm::Slg46824x;
use slg4682x::matrix_input::{GND, VDD};
use slg4682x::matrix_output::MX77_IO7;
use std::thread::sleep;
use std::time::Duration;
use usb4604::{Level, Pio, Usb4604};

fn main() -> Result<()> {
    let usb4604 = Usb4604::open_auto()?;

    // Enable I2C pull-up resistors after USB4604 is booted (i.e., do not tie them to 3V3, otherwise it won't boot)
    let _pull_up_en = usb4604.output(Pio::Pio9, Level::High)?;

    let i2c = usb4604.i2c_bridge()?;
    let mut slg = Slg46824x::new(i2c, 1).expect("");

    // TODO: configure pin as output

    for _ in 0..5 {
        slg.matrix_wire(MX77_IO7, GND)?;
        sleep(Duration::from_millis(500));
        slg.matrix_wire(MX77_IO7, VDD)?;
        sleep(Duration::from_millis(500));
    }
    slg.matrix_wire(MX77_IO7, slg4682x::matrix_input::LUT2_0_DFF0_OUT)?;

    Ok(())
}
