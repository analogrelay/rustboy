use super::{Z80Instruction, Z80RegisterName, registers, instr};
use std::fmt;

#[deriving(Clone)]
pub struct Z80State {
	machine_cycles: int,
	time_cycles: int,
	registers: Z80Registers
}

#[deriving(Clone)]
pub struct Z80Registers {
	a: u8,
	b: u8,
	c: u8,
	d: u8,
	e: u8,
	h: u8,
	l: u8,
	f: u8,
	pc: u16,
	sp: u16
}

pub struct Z80 {
	state: Z80State
}

impl fmt::Show for Z80 {
	fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
		write!(fmt, "Z80 a={a:04} b={b} c={c} d={d} e={e} h={h} l={l} f={f} pc={pc} sp={sp} m={m} t={t}",
			a = self.state.registers.a,
			b = self.state.registers.b,
			c = self.state.registers.c,
			d = self.state.registers.d,
			e = self.state.registers.e,
			h = self.state.registers.h,
			l = self.state.registers.l,
			f = self.state.registers.f,
			pc = self.state.registers.pc,
			sp = self.state.registers.sp,
			m = self.state.machine_cycles,
			t = self.state.time_cycles)
	}
}

impl Z80 {
	pub fn new() -> Z80 {
		Z80 {
			state: Z80State {
				machine_cycles: 0,
				time_cycles: 0,
				registers: Z80Registers {
					a: 0,
					b: 0,
					c: 0,
					d: 0,
					e: 0,
					h: 0,
					l: 0,
					f: 0,
					pc: 0,
					sp: 0
				}
			}
		}
	}

	pub fn exec(&mut self, i: Z80Instruction) {
		dispatch(self, i);
	}

	pub fn dump(&self) -> Z80State {
		self.state.clone()
	}

	fn set_register(&mut self, r: Z80RegisterName, val: u8) {
		match r {
			registers::A => self.state.registers.a = val,
			registers::B => self.state.registers.b = val,
			registers::C => self.state.registers.c = val,
			registers::D => self.state.registers.d = val,
			registers::E => self.state.registers.e = val
		}
	}

	fn advance_clock(&mut self, m: int, t: int) {
		self.state.machine_cycles += m;
		self.state.time_cycles += t;
	}
}

#[inline]
fn dispatch(cpu: &mut Z80, i: Z80Instruction) {
	match i {
		instr::LDrn(r, n) => do_ld_r_n(cpu, r, n)
	}
}

fn do_ld_r_n(cpu: &mut Z80, r: Z80RegisterName, n: u8) {
	cpu.set_register(r, n);
	cpu.advance_clock(2, 8);
}