use anyhow::Result;
use slg4682x::i2c_comm::Slg46824x;
use slg4682x::matrix_input::{GND, IO0_DIGITAL_INPUT, VDD};
use slg4682x::matrix_output::{MX12_IN1_LUT3_1_DIN_DFF4, MX80_IO9};
use usb4604::{Level, Pio, Usb4604};

fn main() -> Result<()> {
    let args = std::env::args().collect::<Vec<_>>();
    let action = args.get(1).expect("gnd or vdd");
    let usb4604 = Usb4604::open_auto()?;

    // Enable I2C pull-up resistors after USB4604 is booted (i.e., do not tie them to 3V3, otherwise it won't boot)
    let _pull_up_en = usb4604.output(Pio::Pio9, Some(Level::High))?;

    let i2c = usb4604.i2c_bridge()?;
    let mut slg = Slg46824x::new(i2c, 1).expect("");

    if action == "gnd" {
        slg.matrix_wire(MX80_IO9, GND)?;
        slg.matrix_wire(MX12_IN1_LUT3_1_DIN_DFF4, VDD)?;
    } else if action == "vdd" {
        slg.matrix_wire(MX80_IO9, VDD)?;
        slg.matrix_wire(MX12_IN1_LUT3_1_DIN_DFF4, IO0_DIGITAL_INPUT)?;
    } else {
        println!("Unknown action: {}, supported: gnd or vdd", action);
    }

    Ok(())
}
