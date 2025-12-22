use anyhow::Result;
use ihex::Record;
use slg4682x::i2c_comm::Slg46824x;
use usb4604::{Level, Pio, Usb4604};

fn main() -> Result<()> {
    let usb4604 = Usb4604::open_auto()?;

    // Enable I2C pull-up resistors after USB4604 is booted (i.e., do not tie them to 3V3, otherwise it won't boot)
    let _pull_up_en = usb4604.output(Pio::Pio9, Level::High)?;

    let i2c = usb4604.i2c_bridge()?;
    let mut slg = Slg46824x::new(i2c, 1).expect("");

    let mut nvm = [0u8; 256];
    // note that reserved bits will differ from Go Configure exported HEX
    slg.read_nvm(0, &mut nvm)?;

    let mut records = Vec::from_iter(nvm.chunks(16).enumerate().map(|(idx, chunk)| Record::Data {
        offset: (idx * 16) as u16,
        value: chunk.to_vec(),
    }));
    records.push(Record::EndOfFile);
    let object = ihex::create_object_file_representation(&records)?;
    println!("{object}");

    Ok(())
}
