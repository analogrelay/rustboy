use super::{Z80RegisterName, Z80RegisterPair};

pub enum Z80Instruction {
	// 8-bit loads
	LDrn(Z80RegisterName, u8),
	LDrr(Z80RegisterName, Z80RegisterName),
	LDrm(Z80RegisterName)
	LDmr(Z80RegisterName),
	LDmn(u8),
	LDadd(Z80RegisterPair),
	LDann(u16),
	LDdda(Z80RegisterPair),
	LDnna(u16),
	LDai,
	LDar,
	LDia,
	LDra,
	
	// 16-bit loads
	LDddnn(Z80RegisterPair, u16),
	LDhlnn(u16),
	LDddm(Z80RegisterPair, u16),
	LDmhl(u16),
	LDmdd(Z80RegisterPair, u16),
	LDsphl,
	PUSHqq(Z80RegisterPair),
	POPqq(Z80RegisterPair),

	// Exchange, Block Transfer, Search Group
	LDI,
	LDIR,
	LDD,
	LDDR,
	CPI,
	CPIR,
	CPD,
	CPDR,

	// 8-bit arithmetic
	ADDar(Z80RegisterName),
	ADDan(u8),
	ADDahl,
	ADCar(Z80RegisterName),
	ADCan(u8),
	ADCahl,
	SUBar(Z80RegisterName),
	SUBan(u8),
	SUBahl,
	SBCar(Z80RegisterName),
	SBCan(u8),
	SBCahl
}