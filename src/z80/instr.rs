use super::Z80RegisterName;

pub enum Z80Instruction {
	LDrn(Z80RegisterName, u8)
}