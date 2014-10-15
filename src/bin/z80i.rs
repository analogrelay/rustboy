extern crate rustboy;

use rustboy::z80::{Z80, instr, registers};

fn main() {
	// Interactive-ish Z80
	let mut cpu = Z80::new();
	println!("{}", cpu);
	cpu.exec(instr::LDrn(registers::A, 42));
	println!("{}", cpu);
}