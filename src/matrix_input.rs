pub struct MatrixInput(u8);

impl MatrixInput {
    pub fn value(&self) -> u8 {
        self.0
    }
}

pub const GND: MatrixInput = MatrixInput(0b0000_0000);
pub const IO0_DIGITAL_INPUT: MatrixInput = MatrixInput(0b0000_0001);
pub const IO1_DIGITAL_INPUT: MatrixInput = MatrixInput(0b0000_0010);
pub const IO2_DIGITAL_INPUT: MatrixInput = MatrixInput(0b0000_0011);
pub const IO3_DIGITAL_INPUT: MatrixInput = MatrixInput(0b0000_0100);
pub const IO4_DIGITAL_INPUT: MatrixInput = MatrixInput(0b0000_0101);
pub const IO5_DIGITAL_INPUT: MatrixInput = MatrixInput(0b0000_0110);
pub const IO8_DIGITAL_INPUT: MatrixInput = MatrixInput(0b0000_0111);
pub const IO9_DIGITAL_INPUT: MatrixInput = MatrixInput(0b0000_1000);
pub const IO10_DIGITAL_INPUT: MatrixInput = MatrixInput(0b0000_1001);
pub const IO11_DIGITAL_INPUT: MatrixInput = MatrixInput(0b0000_1010);
pub const IO12_DIGITAL_INPUT: MatrixInput = MatrixInput(0b0000_1011);
pub const IO13_DIGITAL_INPUT: MatrixInput = MatrixInput(0b0000_1100);
pub const IO14_DIGITAL_INPUT: MatrixInput = MatrixInput(0b0000_1101);
pub const LUT2_0_DFF0_OUT: MatrixInput = MatrixInput(0b0000_1110);
pub const LUT2_1_DFF1_OUT: MatrixInput = MatrixInput(0b0000_1111);
pub const LUT2_2_DFF2_OUT: MatrixInput = MatrixInput(0b0001_0000);
pub const LUT2_3_PGEN_OUT: MatrixInput = MatrixInput(0b0001_0001);
pub const LUT3_0_DFF3_OUT: MatrixInput = MatrixInput(0b0001_0010);
pub const LUT3_1_DFF4_OUT: MatrixInput = MatrixInput(0b0001_0011);
pub const LUT3_2_DFF5_OUT: MatrixInput = MatrixInput(0b0001_0100);
pub const LUT3_3_DFF6_OUT: MatrixInput = MatrixInput(0b0001_0101);
pub const LUT3_4_DFF7_OUT: MatrixInput = MatrixInput(0b0001_0110);
pub const LUT3_5_DFF8_OUT: MatrixInput = MatrixInput(0b0001_0111);
pub const LUT3_6_PIPEDLY_RIPP_CNT_OUT0: MatrixInput = MatrixInput(0b0001_1000);
pub const PIPEDLY_RIPP_CNT_OUT1: MatrixInput = MatrixInput(0b0001_1001);
pub const RIPP_CNT_OUT2: MatrixInput = MatrixInput(0b0001_1010);
pub const EDET_FILTER_OUT: MatrixInput = MatrixInput(0b0001_1011);
pub const PROG_DLY_EDET_OUT: MatrixInput = MatrixInput(0b0001_1100);
pub const MULTFUNC_8BIT_1__DLY_CNT_OUT: MatrixInput = MatrixInput(0b0001_1101);
pub const CKOSC1: MatrixInput = MatrixInput(0b0001_1110);
pub const CKOSC0: MatrixInput = MatrixInput(0b0001_1111);
pub const CKOSC2: MatrixInput = MatrixInput(0b0010_0000);
pub const MULTFUNC_8BIT_2_DLY_CNT_OUT: MatrixInput = MatrixInput(0b0010_0001);
pub const MULTFUNC_8BIT_3_DLY_CNT_OUT: MatrixInput = MatrixInput(0b0010_0010);
pub const MULTFUNC_8BIT_4_DLY_CNT_OUT: MatrixInput = MatrixInput(0b0010_0011);
pub const MULTFUNC_8BIT_5_DLY_CNT_OUT: MatrixInput = MatrixInput(0b0010_0100);
pub const MULTFUNC_8BIT_6_DLY_CNT_OUT: MatrixInput = MatrixInput(0b0010_0101);
pub const MULTFUNC_8BIT_7_DLY_CNT_OUT: MatrixInput = MatrixInput(0b0010_0110);
pub const MULTFUNC_16BIT_0_LUT_DFF_OUT: MatrixInput = MatrixInput(0b0010_0111);
pub const MULTFUNC_8BIT_1_LUT_DFF_OUT: MatrixInput = MatrixInput(0b0010_1000);
pub const MULTFUNC_8BIT_2_LUT_DFF_OUT: MatrixInput = MatrixInput(0b0010_1001);
pub const MULTFUNC_8BIT_3_LUT_DFF_OUT: MatrixInput = MatrixInput(0b0010_1010);
pub const MULTFUNC_8BIT_4_LUT_DFF_OUT: MatrixInput = MatrixInput(0b0010_1011);
pub const MULTFUNC_8BIT_5_LUT_DFF_OUT: MatrixInput = MatrixInput(0b0010_1100);
pub const MULTFUNC_8BIT_6_LUT_DFF_OUT: MatrixInput = MatrixInput(0b0010_1101);
pub const MULTFUNC_8BIT_7_LUT_DFF_OUT: MatrixInput = MatrixInput(0b0010_1110);
pub const MULTFUNC_16BIT_0_DLY_CNT_OUT: MatrixInput = MatrixInput(0b0010_1111);
pub const I2C_VIRTUAL_7_INPUT: MatrixInput = MatrixInput(0b0011_0000);
pub const I2C_VIRTUAL_6_INPUT: MatrixInput = MatrixInput(0b0011_0001);
pub const I2C_VIRTUAL_5_INPUT: MatrixInput = MatrixInput(0b0011_0010);
pub const I2C_VIRTUAL_4_INPUT: MatrixInput = MatrixInput(0b0011_0011);
pub const I2C_VIRTUAL_3_INPUT: MatrixInput = MatrixInput(0b0011_0100);
pub const I2C_VIRTUAL_2_INPUT: MatrixInput = MatrixInput(0b0011_0101);
pub const I2C_VIRTUAL_1_INPUT: MatrixInput = MatrixInput(0b0011_0110);
pub const I2C_VIRTUAL_0_INPUT: MatrixInput = MatrixInput(0b0011_0111);
pub const ACMP0L_OUT: MatrixInput = MatrixInput(0b0011_1010);
pub const ACMP1L_OUT: MatrixInput = MatrixInput(0b0011_1011);
pub const CKOSC1_2ND: MatrixInput = MatrixInput(0b0011_1100);
pub const CKOSC0_2ND: MatrixInput = MatrixInput(0b0011_1101);
pub const POR_OUT: MatrixInput = MatrixInput(0b0011_1110);
pub const VDD: MatrixInput = MatrixInput(0b0011_1111);
