#![allow(dead_code)]
use kernel::common::regs::{ReadOnly};

/******************************************************************************
 * Radio and data commands bitfields
*******************************************************************************/

bitfield! {
    #[derive(Copy, Clone)]
    pub struct RfcTrigger(u8);
    impl Debug;
    pub _trigger_type, _set_trigger_type : 3, 0;
    pub _enable_cmd, _set_enable_cmd      : 4;
    pub _trigger_no, _set_trigger_no      : 6, 5;
    pub _past_trigger, _set_past_trigger  : 7;

}

bitfield! {
    #[derive(Copy, Clone)]
    pub struct RfcCondition(u8);
    impl Debug;
    pub _rule, set_rule : 3, 0;
    pub _skip, _set_skip : 7, 4;
}

bitfield! {
    #[derive(Copy, Clone)]
    pub struct RfcSetupConfig(u16);
    impl Debug;
    pub _frontend_mode, set_frontend_mode: 2, 0;
    pub _bias_mode, set_bias_mode: 3;
    pub _analog_cfg_mode, _set_analog_config_mode: 9, 4;
    pub _no_fs_powerup, _set_no_fs_powerup: 10;
}
/******************************************************************************
 * Radio Command structure headers, bitfields, and partial settings for the
 * bitfields
 *
*******************************************************************************/

#[derive(Clone, Copy)]
pub enum TriggerType {
    Now = 0,
    Never = 1,
    AbsTime = 2,
    Submit = 3,
    RelStart = 4, 
    RelPrevStart = 5, 
    RelFirstStart = 6,
    RelPrevEnd = 7,
    RelEvt1 = 8,
    RelEvt2 = 9,
    External = 10,
}


#[derive(Clone, Copy)]
pub enum ConditionRules {
    Always = 0,
    Never = 1,
    StopOnFalse = 2,
    StopOnTrue = 3,
    SkipOnFalse = 4,
    SkipOnTrue = 5,
}

pub struct DataEntryQueue {
    p_curr_entry: *mut u32,
    p_last_entry: *mut u32,
}

/******************************************************************************
 * Radio Commands
 *
*******************************************************************************/
/*
    RFC Immediate commands
*/
pub const RFC_CMD0: u16 = 0x607;
pub const RFC_PING: u16 = 0x406;
pub const RFC_BUS_REQUEST: u16 = 0x40E;
pub const RFC_START_RAT_TIMER: u16 = 0x080A;
pub const RFC_STOP_RAT_TIMER: u16 = 0x0809;
pub const RFC_SETUP: u16 = 0x0802;
pub const RFC_STOP: u16 = 0x0402;
pub const RFC_FS_POWERDOWN: u16 = 0x080D;

pub struct DirectCommand{
    pub command_no: u16,
    pub params: u16,
}

impl DirectCommand {
    pub const fn new(command_no: u16, params: u16) -> DirectCommand {
        DirectCommand {
            command_no,
            params,
        }
    }
}

#[repr(C)]
pub struct CmdRadioSetup {
    pub command_no:     u16,
    pub status:         u16,
    pub p_next_op:      u32, // Pointer to next command
    pub start_time:     u32,
    pub start_trigger:  u8,
    pub condition:      RfcCondition,
    pub mode:           u8,
    pub io_divider:     u8,
    pub config:         RfcSetupConfig,
    pub tx_power:       u16,
    pub reg_override:   u32,
}

impl CmdRadioSetup {
    pub fn new(reg_override: u32, mode: u8, tx_power: u16) -> CmdRadioSetup {
        CmdRadioSetup {
            command_no: 0x802,
            status: 0,
            p_next_op: 0,
            start_time: 0,
            start_trigger: 0,
            condition: {
                let mut cond = RfcCondition(0);
                cond.set_rule(0x01);
                cond 
            },
            mode,
            io_divider: 0,
            config: {
                let mut cfg = RfcSetupConfig(0);
                cfg.set_frontend_mode(0);
                cfg.set_bias_mode(false);
                cfg 
            },
            tx_power,
            reg_override,
        }
    }
}

#[repr(C)]
pub struct CmdCommon {
    pub command_no:     ReadOnly<u16>,
    pub status:         ReadOnly<u16>,
    pub p_next_op:      ReadOnly<u32>,
    pub start_time:     ReadOnly<u32>,
    pub start_trigger:  ReadOnly<u8>,
    pub condition:      RfcCondition,
}

#[repr(C)]
pub struct CmdNop {
    pub command_no:     u16, //0x0801
    pub status:         u16,
    pub p_next_op:      u32,
    pub start_time:     u32,
    pub start_trigger:  u8,
    pub condition:      RfcCondition,
}

#[repr(C)]
pub struct CmdFSPowerup {
    pub command_no:     u16, //0x080C
    pub status:         u16,
    pub p_next_op:      u32,
    pub start_time:     u32,
    pub start_trigger:  u8,
    pub condition:      RfcCondition,
    pub reserved:       u16,
    pub reg_override:   u32,
}

#[repr(C)]
pub struct CmdFSPowerdown {
    pub command_no:     u16, //0x080D
    pub status:         u16,
    pub p_next_op:      u32,
    pub start_time:     u32,
    pub start_trigger:  u8,
    pub condition:      RfcCondition,
}

impl CmdFSPowerdown {
    pub fn new() -> CmdFSPowerdown {
        CmdFSPowerdown {
            command_no: 0x080D,
            status: 0,
            p_next_op: 0, 
            start_time: 0,
            start_trigger: 0,
            condition: {
                let mut cond = RfcCondition(0);
                cond.set_rule(0x01);
                cond 
            },
        }
    }
}

#[repr(C)]
pub struct CmdFS {
    pub command_no:     u16, // 0x0803
    pub status:         u16,
    pub p_next_op:      u32,
    pub start_time:     u32,
    pub start_trigger:  u8,
    pub condition:      RfcCondition,
    pub fract_freq:     u16,
    pub synth_conf:     u8,
    _reserved:          [u8; 5],
}

#[repr(C)]
pub struct CmdFSOff {
    pub command_no:     u16, // 0x0804
    pub status:         u16,
    pub p_next_op:      u32,
    pub start_time:     u32,
    pub start_trigger:  u8,
    pub condition:      RfcCondition,   
}

#[repr(C)]
pub struct CmdRxTest {
    pub command_no:     u16, // 0x0807
    pub status:         u16,
    pub p_next_op:      u32,
    pub start_time:     u32,
    pub start_trigger:  u8,
    pub condition:      RfcCondition,
    pub config:         u8,
    pub end_trigger:    u8,
    pub sync_word:      u32,
    pub end_time:       u32,
}

#[repr(C)]
pub struct CmdTxTest {
    pub command_no:     u16, // 0x0808
    pub status:         u16,
    pub p_next_op:      u32,
    pub start_time:     u32,
    pub start_trigger:  u8,
    pub condition:      RfcCondition,
    pub config:         u8,
    _reserved_a:        u8,
    pub tx_word:        u16,
    _reserved_b:        u8,
    pub end_trigger:    u8,
    pub sync_word:      u32,
    pub end_time:       u32,
}

#[repr(C)]
pub struct CmdSyncStopRat {
    pub command_no:     u16, // 0x0809
    pub status:         u16,
    pub p_next_op:      u32,
    pub start_time:     u32,
    pub start_trigger:  u8,
    pub condition:      RfcCondition,
    _reserved:          u16,
    pub rat0:           u32,
}

impl CmdSyncStopRat {
    pub fn new(rat: u32) -> CmdSyncStopRat {
        CmdSyncStopRat {
            command_no: 0x0809,
            status: 0,
            p_next_op: 0,
            start_time: 0,
            start_trigger: 0,
            condition: {
                let mut cond = RfcCondition(0);
                cond.set_rule(0x01);
                cond 
            },
            _reserved: 0x0000,
            rat0: rat,
        }
    }
}

#[repr(C)]
pub struct CmdSyncStartRat {
    pub command_no:     u16, // 0x080A
    pub status:         u16,
    pub p_next_op:      u32,
    pub start_time:     u32,
    pub start_trigger:  u8,
    pub condition:      RfcCondition,
    _reserved:          u16,
    pub rat0:           u32,
}

impl CmdSyncStartRat {
    pub fn new(rat: u32) -> CmdSyncStartRat {
        CmdSyncStartRat {
            command_no: 0x0809,
            status: 0,
            p_next_op: 0,
            start_time: 0,
            start_trigger: 0,
            condition: {
                let mut cond = RfcCondition(0);
                cond.set_rule(0x01);
                cond
            },
            _reserved: 0x0000,
            rat0: rat,
        }
    }
}

