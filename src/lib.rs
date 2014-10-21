#![feature(phase)]
#![feature(macro_rules)]

#[phase(plugin, link)]
extern crate log;

pub mod z80;