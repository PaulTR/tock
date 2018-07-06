#![feature(asm, concat_idents, const_fn, const_cell_new, try_from)]
#![no_std]
#![crate_name = "cc13x2"]
#![crate_type = "rlib"]
extern crate cc26xx;
// extern crate cc26x2;
extern crate cortexm4;
#[allow(unused_imports)]
#[macro_use]
extern crate kernel;
#[macro_use]
extern crate bitfield;

pub mod aon;
pub mod chip;
pub mod commands;
pub mod crt1;
pub mod prcm;
pub mod rat;
pub mod rfc;
pub mod rtc;

pub use crt1::init;
