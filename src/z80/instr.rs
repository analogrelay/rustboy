use super::{Z80RegisterName, Z80RegisterPair};

pub enum Z80Instruction {
	// 8-bit loads
	LDrr(Z80RegisterName, Z80RegisterName),
	LDrHLm(Z80RegisterName),
	LDHLSPn(i8)
}