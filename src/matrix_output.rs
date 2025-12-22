use std::ops::RangeInclusive;

pub struct MatrixOutput(RangeInclusive<usize>);

impl MatrixOutput {
    pub fn value(&self) -> RangeInclusive<usize> {
        self.0.clone()
    }
}

/// IN0 of LUT2_0 or Clock Input of DFF0
pub const MX0_IN0_LUT2_0_CLKIN_DFF0: MatrixOutput = MatrixOutput(5..=0);
/// IN1 of LUT2_0 or Data Input of DFF0
pub const MX1_IN1_LUT2_0_DIN_DFF0: MatrixOutput = MatrixOutput(11..=6);
/// IN0 of LUT2_3 or Clock Input of PGen
pub const MX2_IN0_LUT2_3_CLKIN_PGEN: MatrixOutput = MatrixOutput(17..=12);
/// IN1 of LUT2_3 or nRST of PGen
pub const MX3_IN1_LUT2_3_NRST_PGEN: MatrixOutput = MatrixOutput(23..=18);
/// IN0 of LUT2_1 or Clock Input of DFF1
pub const MX4_IN0_LUT2_1_CLKIN_DFF1: MatrixOutput = MatrixOutput(29..=24);
/// IN1 of LUT2_1 or Data Input of DFF1
pub const MX5_IN1_LUT2_1_DIN_DFF1: MatrixOutput = MatrixOutput(35..=30);
/// IN0 of LUT2_2 or Clock Input of DFF2
pub const MX6_IN0_LUT2_2_CLKIN_DFF2: MatrixOutput = MatrixOutput(41..=36);
/// IN1 of LUT2_2 or Data Input of DFF2
pub const MX7_IN1_LUT2_2_DIN_DFF2: MatrixOutput = MatrixOutput(47..=42);
/// IN0 of LUT3_0 or Clock Input of DFF3
pub const MX8_IN0_LUT3_0_CLKIN_DFF3: MatrixOutput = MatrixOutput(53..=48);
/// IN1 of LUT3_0 or Data Input of DFF3
pub const MX9_IN1_LUT3_0_DIN_DFF3: MatrixOutput = MatrixOutput(59..=54);
/// IN2 of LUT3_0 or nRST(nSET) of DFF3
pub const MX10_IN2_LUT3_0_NRST_NSET_DFF3: MatrixOutput = MatrixOutput(65..=60);
/// IN0 of LUT3_1 or Clock Input of DFF4
pub const MX11_IN0_LUT3_1_CLKIN_DFF4: MatrixOutput = MatrixOutput(71..=66);
/// IN1 of LUT3_1 or Data Input of DFF4
pub const MX12_IN1_LUT3_1_DIN_DFF4: MatrixOutput = MatrixOutput(77..=72);
/// IN2 of LUT3_1 or nRST(nSET) of DFF4
pub const MX13_IN2_LUT3_1_NRST_NSET_DFF4: MatrixOutput = MatrixOutput(83..=78);
/// IN0 of LUT3_2 or Clock Input of DFF5
pub const MX14_IN0_LUT3_2_CLKIN_DFF5: MatrixOutput = MatrixOutput(89..=84);
/// IN1 of LUT3_2 or Data Input of DFF5
pub const MX15_IN1_LUT3_2_DIN_DFF5: MatrixOutput = MatrixOutput(95..=90);
/// IN2 of LUT3_2 or nRST(nSET) of DFF5
pub const MX16_IN2_LUT3_2_NRST_NSET_DFF5: MatrixOutput = MatrixOutput(101..=96);
/// IN0 of LUT3_3 or Clock Input of DFF6
pub const MX17_IN0_LUT3_3_CLKIN_DFF6: MatrixOutput = MatrixOutput(107..=102);
/// IN1 of LUT3_3 or Data Input of DFF6
pub const MX18_IN1_LUT3_3_DIN_DFF6: MatrixOutput = MatrixOutput(113..=108);
/// IN2 of LUT3_3 or nRST(nSET) of DFF6
pub const MX19_IN2_LUT3_3_NSRT_NSET_DFF6: MatrixOutput = MatrixOutput(119..=114);
/// IN0 of LUT3_4 or Clock Input of DFF7
pub const MX20_IN0_LUT3_4_CLKIN_DFF7: MatrixOutput = MatrixOutput(125..=120);
/// IN1 of LUT3_4 or Data Input of DFF7
pub const MX21_IN1_LUT3_4_DIN_DFF7: MatrixOutput = MatrixOutput(131..=126);
/// IN2 of LUT3_4 or nRST(nSET) of DFF7
pub const MX22_IN2_LUT3_4_NRST_NSET_DFF7: MatrixOutput = MatrixOutput(137..=132);
/// IN0 of LUT3_5 or Clock Input of DFF8
pub const MX23_IN0_LUT3_5_CLKIN_DFF8: MatrixOutput = MatrixOutput(143..=138);
/// IN1 of LUT3_5 or Data Input of DFF8
pub const MX24_IN1_LUT3_5_DIN_DFF8: MatrixOutput = MatrixOutput(149..=144);
/// IN2 of LUT3_5 or nRST(nSET) of DFF8
pub const MX25_IN2_LUT3_5_NRST_NSET_DFF8: MatrixOutput = MatrixOutput(155..=150);
/// IN0 of LUT3_6 or Input of Pipe Delay or UP Signal of RIPP CNT
pub const MX26_IN0_LUT3_6_IN_PDLY_UP_RIPP_CNT: MatrixOutput = MatrixOutput(161..=156);
/// IN1 of LUT3_6 or nRST of Pipe Delay or STB of RIPP CNT
pub const MX27_IN1_LUT3_6_NRST_PDLY_STB_RIPP_CNT: MatrixOutput = MatrixOutput(167..=162);
/// IN2 of LUT3_6 or Clock of Pipe Delay_RIPP_CNT
pub const MX28_IN2_LUT3_6_CLK_PDLY_RIPP_CNT: MatrixOutput = MatrixOutput(173..=168);
/// MULTFUNC_16BIT_0: IN0 of LUT4_0 or Clock Input of DFF9; Delay0 Input (or Counter0 nRST/SET Input)
pub const MX30: MatrixOutput = MatrixOutput(185..=180);
/// MULTFUNC_16BIT_0: IN1 of LUT4_0 or nRST of DFF9; Delay0 Input (or Counter0 nRST Input) or Delay/Counter0 External Clock Source
pub const MX31: MatrixOutput = MatrixOutput(191..=186);
/// MULTFUNC_16BIT_0: IN2 of LUT4_0 or nSET of DFF9; Delay0 Input (or Counter0 nRST Input) or Delay/Counter0 External Clock Source or KEEP Input of FSM0
pub const MX32: MatrixOutput = MatrixOutput(197..=192);
/// MULTFUNC_16BIT_0: IN3 of LUT4_0 or Data Input of DFF9; Delay0 Input (or Counter0 nRST Input) or UP Input of FSM0
pub const MX33: MatrixOutput = MatrixOutput(203..=198);
/// MULTFUNC_8BIT_1: IN0 of LUT3_7 or Clock Input of DFF10; Delay1 Input (or Counter1 nRST Input)
pub const MX34: MatrixOutput = MatrixOutput(209..=204);
/// MULTFUNC_8BIT_1: IN1 of LUT3_7 or nRST (nSET) of DFF10; Delay1 Input (or Counter1 nRST Input) or Delay/Counter1 External Clock Source
pub const MX35: MatrixOutput = MatrixOutput(215..=210);
/// MULTFUNC_8BIT_1: IN2 of LUT3_7 or Data Input of DFF10; Delay1 Input (or Counter1 nRST Input)
pub const MX36: MatrixOutput = MatrixOutput(221..=216);
/// MULTFUNC_8BIT_2: IN0 of LUT3_8 or Clock Input of DFF11; Delay2 Input (or Counter2 nRST Input);
pub const MX37: MatrixOutput = MatrixOutput(227..=222);
/// MULTFUNC_8BIT_2: IN1 of LUT3_8 or nRST (nSET) of DFF11; Delay2 Input (or Counter2 nRST Input) or Delay/Counter2 External Clock Source
pub const MX38: MatrixOutput = MatrixOutput(233..=228);
/// MULTFUNC_8BIT_2: IN2 of LUT3_8 or Data Input of DFF11; Delay2 Input (or Counter2 nRST Input)
pub const MX39: MatrixOutput = MatrixOutput(239..=234);
/// MULTFUNC_8BIT_3: IN0 of LUT3_9 or Clock Input of DFF12; Delay3 Input (or Counter3 nRST Input)
pub const MX40: MatrixOutput = MatrixOutput(245..=240);
/// MULTFUNC_8BIT_3: IN1 of LUT3_9 or nRST (nSET) of DFF12; Delay3 Input (or Counter3 nRST Input) or Delay/Counter3 External Clock Source
pub const MX41: MatrixOutput = MatrixOutput(251..=246);
/// MULTFUNC_8BIT_3: IN2 of LUT3_9 or Data Input of DFF12; Delay3 Input (or Counter3 nRST Input)
pub const MX42: MatrixOutput = MatrixOutput(257..=252);
/// MULTFUNC_8BIT_4: IN0 of LUT3_10 or Clock Input of DFF13; Delay4 Input (or Counter4 nRST Input)
pub const MX43: MatrixOutput = MatrixOutput(263..=258);
/// MULTFUNC_8BIT_4: IN1 of LUT3_10 or nRST (nSET) of DFF13; Delay4 Input (or Counter4 nRST Input) or Delay/Counter4 External Clock Source
pub const MX44: MatrixOutput = MatrixOutput(269..=264);
/// MULTFUNC_8BIT_4: IN2 of LUT3_10 or Data Input of DFF13; Delay4 Input (or Counter4 nRST Input)
pub const MX45: MatrixOutput = MatrixOutput(275..=270);
/// MULTFUNC_8BIT_5: IN0 of LUT3_11 or Clock Input of DFF14; Delay5 Input (or Counter5 nRST Input)
pub const MX46: MatrixOutput = MatrixOutput(281..=276);
/// MULTFUNC_8BIT_5: IN1 of LUT3_11 or nRST (nSET) of DFF14; Delay5 Input (or Counter5 nRST Input) or Delay/Counter5 External Clock Source
pub const MX47: MatrixOutput = MatrixOutput(287..=282);
/// MULTFUNC_8BIT_5: IN2 of LUT3_11 or Data Input of DFF14; Delay5 Input (or Counter5 nRST Input)
pub const MX48: MatrixOutput = MatrixOutput(293..=288);
/// MULTFUNC_8BIT_6: IN0 of LUT3_12 or Clock Input of DFF15; Delay6 Input (or Counter6 nRST Input)
pub const MX49: MatrixOutput = MatrixOutput(299..=294);
/// MULTFUNC_8BIT_6: IN1 of LUT3_12 or nRST (nSET) of DFF15; Delay6 Input (or Counter6 nRST Input) or Delay/Counter6 External Clock Source
pub const MX50: MatrixOutput = MatrixOutput(305..=300);
/// MULTFUNC_8BIT_6: IN2 of LUT3_12 or Data Input of DFF15; Delay6 Input (or Counter6 nRST Input)
pub const MX51: MatrixOutput = MatrixOutput(311..=306);
/// MULTFUNC_8BIT_7: IN0 of LUT3_13 or Clock Input of DFF16; Delay7 Input (or Counter7 nRST Input)
pub const MX52: MatrixOutput = MatrixOutput(317..=312);
/// MULTFUNC_8BIT_7: IN1 of LUT3_13 or nRST (nSET) of DFF16; Delay7 Input (or Counter7 nRST Input) or Delay/Counter7 External Clock Source
pub const MX53: MatrixOutput = MatrixOutput(323..=318);
/// MULTFUNC_8BIT_7: IN2 of LUT3_13 or Data Input of DFF16; Delay7 Input (or Counter7 nRST Input)
pub const MX54: MatrixOutput = MatrixOutput(329..=324);
/// Filter/Edge detect input
pub const MX55_FLT_EDGE_DETECTOR: MatrixOutput = MatrixOutput(335..=330);
/// Programmable delay/edge detect input
pub const MX56_DL_EDGE_DETECTOR: MatrixOutput = MatrixOutput(341..=336);
/// OSC2 ENABLE from matrix
pub const MX57_OSC2_EN: MatrixOutput = MatrixOutput(347..=342);
/// OSC0 ENABLE from matrix
pub const MX58_OSC0_EN: MatrixOutput = MatrixOutput(353..=348);
/// OSC1 ENABLE matrix
pub const MX59_OSC1_EN: MatrixOutput = MatrixOutput(359..=354);
/// Vref PD from matrix
pub const MX60_VREF_PD: MatrixOutput = MatrixOutput(365..=360);
/// BG power-down from matrix
pub const MX61_BG_PWRDN: MatrixOutput = MatrixOutput(371..=366);
/// PWR UP of ACMP0L from matrix
pub const MX64_ACMP0L_PWRUP: MatrixOutput = MatrixOutput(389..=384);
/// PWR UP of ACMP1L from matrix
pub const MX65_ACMP1L_PWRUP: MatrixOutput = MatrixOutput(395..=390);
/// IO0 Digital Output
pub const MX67_IO0: MatrixOutput = MatrixOutput(407..=402);
/// IO1 Digital Output
pub const MX68_IO1: MatrixOutput = MatrixOutput(413..=408);
/// IO1 Digital Output OE
pub const MX69_IO1_OE: MatrixOutput = MatrixOutput(419..=414);
/// IO2 Digital Output
pub const MX70_IO2: MatrixOutput = MatrixOutput(425..=420);
/// IO3 Digital Output
pub const MX71_IO3: MatrixOutput = MatrixOutput(431..=426);
/// IO4 Digital Output
pub const MX72_IO4: MatrixOutput = MatrixOutput(437..=432);
/// IO4 Digital Output OE
pub const MX73_IO4_OE: MatrixOutput = MatrixOutput(443..=438);
/// IO5 Digital Output
pub const MX74_IO5: MatrixOutput = MatrixOutput(449..=444);
/// IO5 Digital Output OE
pub const MX75_IO5_OE: MatrixOutput = MatrixOutput(455..=450);
/// IO6 Digital Output
pub const MX76_IO6: MatrixOutput = MatrixOutput(461..=456);
/// IO7 Digital Output
pub const MX77_IO7: MatrixOutput = MatrixOutput(467..=462);
/// IO8 Digital Output
pub const MX78_IO8: MatrixOutput = MatrixOutput(473..=468);
/// IO8 Digital Output OE
pub const MX79_IO8_OE: MatrixOutput = MatrixOutput(479..=474);
/// IO9 Digital Output
pub const MX80_IO9: MatrixOutput = MatrixOutput(485..=480);
/// IO9 Digital Output OE
pub const MX81_IO9_OE: MatrixOutput = MatrixOutput(491..=486);
/// IO10 Digital Output
pub const MX82_IO10: MatrixOutput = MatrixOutput(497..=492);
/// IO10 Digital Output OE
pub const MX83_IO10_OE: MatrixOutput = MatrixOutput(503..=498);
/// IO11 Digital Output
pub const MX84_IO11: MatrixOutput = MatrixOutput(509..=504);
/// IO11 Digital Output OE
pub const MX85_IO11_OE: MatrixOutput = MatrixOutput(515..=510);
/// IO12 Digital Output
pub const MX86_IO12: MatrixOutput = MatrixOutput(521..=516);
/// IO12 Digital Output OE
pub const MX87_IO12_OE: MatrixOutput = MatrixOutput(527..=522);
/// IO13 Digital Output
pub const MX88_IO13: MatrixOutput = MatrixOutput(533..=528);
/// IO13 Digital Output OE
pub const MX89_IO13_OE: MatrixOutput = MatrixOutput(539..=534);
/// IO14 Digital Output
pub const MX90_IO14: MatrixOutput = MatrixOutput(545..=540);
/// IO14 Digital Output OE
pub const MX91_IO14_OE: MatrixOutput = MatrixOutput(551..=546);
/// Matrix OUT 94
pub const MX94: MatrixOutput = MatrixOutput(569..=564);
/// Matrix OUT 95
pub const MX95: MatrixOutput = MatrixOutput(575..=570);
