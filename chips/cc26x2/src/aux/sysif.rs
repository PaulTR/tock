use kernel::common::registers::{ReadOnly, ReadWrite};
use kernel::common::StaticRef;

use memory_map::AUX_SYSIF_BASE;

pub const REGISTERS: StaticRef<Registers> =
    unsafe { StaticRef::new(AUX_SYSIF_BASE as *const Registers) };

pub union SimpleAckReq {
    pub ack: ReadOnly<u32, Ack::Register>,
    pub req: ReadWrite<u32, Req::Register>,
}

impl SimpleAckReq {
    /// Returns the control register modality
    pub fn ack(&self) -> &ReadOnly<u32, Ack::Register> {
        unsafe { &self.ack }
    }

    /// Returns the status register modality
    pub fn req(&self) -> &ReadWrite<u32, Req::Register> {
        unsafe { &self.req }
    }
}

#[repr(C)]
pub struct Registers {
    pub op_mode_req: ReadWrite<u32, OpModeReq::Register>,
    pub op_mode_ack: ReadOnly<u32, OpModeAck::Register>,
    prog_wu0_cfg: ReadWrite<u32, WUCfg::Register>,
    _prog_wu1_cfg: ReadWrite<u32, WUCfg::Register>,
    _prog_wu2_cfg: ReadWrite<u32, WUCfg::Register>,
    _prog_wu3_cfg: ReadWrite<u32, WUCfg::Register>,
    _swwutrig: ReadOnly<u32>,
    _wu_flags: ReadOnly<u32, WUFlags::Register>,
    _wu_flags_clr: ReadWrite<u32, WUFlags::Register>,
    wu_gate: ReadWrite<u32, WUGate::Register>,
    _vec_cfg: [ReadOnly<u32>; 8],
    _evsyncrate: ReadOnly<u32>,
    _peroprate: ReadOnly<u32>,
    pub adc_clk_ctl: SimpleAckReq,
    _tdc_clk_ctl: ReadOnly<u32>,
    _tdc_ref_clk_ctl: ReadOnly<u32>,
    _timer2: [ReadOnly<u32>; 4],
    _reserved: ReadOnly<u32>,
    _clk_shift_det: ReadOnly<u32>,
    _recharge: [ReadOnly<u32>; 2],
    _rtc_subsec_inc: [ReadOnly<u32>; 6],
    _batmon_bat: ReadOnly<u32>,
    _reserved2: ReadOnly<u32>,
    _batmon_temp: ReadOnly<u32>,
    _timer_halt: ReadOnly<u32>,
    _reserved3: [ReadOnly<u32>; 3],
    _timer2_bridge: ReadOnly<u32>,
    _sw_pwr_prof: ReadOnly<u32>,
}

register_bitfields! [
    u32,
    Req [
        CLOCK         OFFSET(0) NUMBITS(1) [
            Disable = 0,
            Enable = 1
        ]
    ],
    Ack [
        CLOCK OFFSET(1) NUMBITS(1) [
            Disabled = 0,
            Enabled = 1
        ]
    ],
    OpModeAck [
        CLOCK         OFFSET(0) NUMBITS(2) [
            Disabled = 0,
            Enabled = 1
        ]
    ],
    OpModeReq [
        CLOCK         OFFSET(0) NUMBITS(2) [
            Active = 0x0,
            LowPower = 0x1,
            PowerDownActive = 0x2,
            PowerDownLowPower = 0x3
        ]
    ],
    WUCfg [
        POL         OFFSET(7) NUMBITS(1) [],
        EN          OFFSET(6) NUMBITS(1) [],
        WU_SRC      OFFSET(0) NUMBITS(6) [
            AuxIO0              = 0b000000,
            AuxIO1              = 0b000001,
            AuxIO2              = 0b000010,
            AuxIO3              = 0b000011,
            AuxIO4              = 0b000100,
            AuxIO5              = 0b000101,
            AuxIO6              = 0b000110,
            AuxIO7              = 0b000111,
            AuxIO8              = 0b001000,
            AuxIO9              = 0b001001,
            AuxIO10             = 0b001010,
            AuxIO11             = 0b001011,
            AuxIO12             = 0b001100,
            AuxIO13             = 0b001101,
            AuxIO14             = 0b001110,
            AuxIO15             = 0b001111,
            AuxIO16             = 0b010000,
            AuxIO17             = 0b010001,
            AuxIO18             = 0b010010,
            AuxIO19             = 0b010011,
            AuxIO20             = 0b010100,
            AuxIO21             = 0b010101,
            AuxIO22             = 0b010110,
            AuxIO23             = 0b010111,
            AuxIO24             = 0b011000,
            AuxIO25             = 0b011001,
            AuxIO26             = 0b011010,
            AuxIO27             = 0b011011,
            AuxIO28             = 0b011100,
            AuxIO29             = 0b011101,
            AuxIO30             = 0b011110,
            AuxIO31             = 0b011111,
            ManuelEv            = 0b100000,
            AonRtcCh2           = 0b100001,
            AonRtcCh2Dly        = 0b100010,
            AonRtc4khz          = 0b100011,
            AonBatBatUpd        = 0b100100,
            AonBatTempUpd       = 0b100101,
            SclkLf              = 0b100110,
            PwrDwn              = 0b100111,
            McuActive           = 0b101000,
            VddrRecharge        = 0b101001,
            AclkRef             = 0b101010,
            McuEv               = 0b101011,
            McuObsMux0          = 0b101100,
            McuObsMux1          = 0b101101,
            AuxCompA            = 0b101110,
            AuxCompB            = 0b101111,
            AuxTimer2Ev0        = 0b110000,
            AuxTimer2Ev1        = 0b110001,
            AuxTimer2Ev2        = 0b110010,
            AuxTimer2Ev3        = 0b110011,
            AuxTimer2Pulse      = 0b110100,
            AuxTimer1Ev         = 0b110101,
            AuxTimer0Ev         = 0b110110,
            AuxTdcDone          = 0b110111,
            AuxIsrcReset        = 0b111000,
            AuxAdcDone          = 0b111001,
            AuxAdcIrq           = 0b111010,
            AuxAdcFifoFull      = 0b111011,
            AuxAdcFifoNotEmpty  = 0b111100,
            AuxSmphAutoTakeDone = 0b111101,
            NoEvent             = 0b111110,
            NoEvent2            = 0b111111
        ]
    ],
    WUFlags [
        SW_WU3      OFFSET(7) NUMBITS(1) [],
        SW_WU2      OFFSET(6) NUMBITS(1) [],
        SW_WU1      OFFSET(5) NUMBITS(1) [],
        SW_WU0      OFFSET(4) NUMBITS(1) [],
        PROG_WU3    OFFSET(3) NUMBITS(1) [],
        PROG_WU2    OFFSET(2) NUMBITS(1) [],
        PROG_WU1    OFFSET(1) NUMBITS(1) [],
        PROG_WU0    OFFSET(0) NUMBITS(1) []
    ],
    WUGate [
        EN          OFFSET(0) NUMBITS(1) []
    ]
];

pub enum WakeUpSource {
    AuxIO0 = 0b000000,
    AuxIO1 = 0b000001,
    AuxIO2 = 0b000010,
    AuxIO3 = 0b000011,
    AuxIO4 = 0b000100,
    AuxIO5 = 0b000101,
    AuxIO6 = 0b000110,
    AuxIO7 = 0b000111,
    AuxIO8 = 0b001000,
    AuxIO9 = 0b001001,
    AuxIO10 = 0b001010,
    AuxIO11 = 0b001011,
    AuxIO12 = 0b001100,
    AuxIO13 = 0b001101,
    AuxIO14 = 0b001110,
    AuxIO15 = 0b001111,
    AuxIO16 = 0b010000,
    AuxIO17 = 0b010001,
    AuxIO18 = 0b010010,
    AuxIO19 = 0b010011,
    AuxIO20 = 0b010100,
    AuxIO21 = 0b010101,
    AuxIO22 = 0b010110,
    AuxIO23 = 0b010111,
    AuxIO24 = 0b011000,
    AuxIO25 = 0b011001,
    AuxIO26 = 0b011010,
    AuxIO27 = 0b011011,
    AuxIO28 = 0b011100,
    AuxIO29 = 0b011101,
    AuxIO30 = 0b011110,
    AuxIO31 = 0b011111,
    ManuelEv = 0b100000,
    AonRtcCh2 = 0b100001,
    AonRtcCh2Dly = 0b100010,
    AonRtc4khz = 0b100011,
    AonBatBatUpd = 0b100100,
    AonBatTempUpd = 0b100101,
    SclkLf = 0b100110,
    PwrDwn = 0b100111,
    McuActive = 0b101000,
    VddrRecharge = 0b101001,
    AclkRef = 0b101010,
    McuEv = 0b101011,
    McuObsMux0 = 0b101100,
    McuObsMux1 = 0b101101,
    AuxCompA = 0b101110,
    AuxCompB = 0b101111,
    AuxTimer2Ev0 = 0b110000,
    AuxTimer2Ev1 = 0b110001,
    AuxTimer2Ev2 = 0b110010,
    AuxTimer2Ev3 = 0b110011,
    AuxTimer2Pulse = 0b110100,
    AuxTimer1Ev = 0b110101,
    AuxTimer0Ev = 0b110110,
    AuxTdcDone = 0b110111,
    AuxIsrcReset = 0b111000,
    AuxAdcDone = 0b111001,
    AuxAdcIrq = 0b111010,
    AuxAdcFifoFull = 0b111011,
    AuxAdcFifoNotEmpty = 0b111100,
    AuxSmphAutoTakeDone = 0b111101,
    NoEvent = 0b111110,
    NoEvent2 = 0b111111,
}

pub const WUMODE_A: u8 = 0;
pub const WUMODE_LP: u8 = 1;
pub const WUMODE_PDA: u8 = 2;
pub const WUMODE_PDLP: u8 = 3;

pub enum Polarity {
    High,
    Low,
}
