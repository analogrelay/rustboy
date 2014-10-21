use super::{Z80RegisterName, Z80RegisterPair};

pub enum Z80Instruction {
	// 8-bit loads
	LDrn(Z80RegisterName, u8),
    LDrr(Z80RegisterName, Z80RegisterName),
	LDrHLm(Z80RegisterName),
	LDHLmr(Z80RegisterName),
    LDHLmn(u8),

    // 16-bit loads
    LDrrmA(Z80RegisterPair),
    LDmmA(u16),
    LDArrm(Z80RegisterPair),
    LDAmm(u16),
    LDrrnn(Z80RegisterPair, u16),
    LDHLmm(u16),
    LDmmHL(u16),
    LDHLIA,
    LDAHLI,
    LDHLDA,
    LDAHLD,
    LDAIOn(u8),
    LDIOnA(u8),
    LDAIOC,
    LDIOCA,
    LDHLSPn(i8),

    // Other
    SWAP(Z80RegisterName),
    
    // Arithmetic
	ADDr(Z80RegisterName),
    ADDmHL,
    ADDn(u8),
    ADDHLrr(Z80RegisterPair),
    ADDSPn(i8),
	ADCr(Z80RegisterName),
    ADCmHL,
    ADCn(u8),
    SUBr(Z80RegisterName),
    SUBmHL,
    SUBn(u8),
    SBCr(Z80RegisterName),
    SBCmHL,
    SBCn(u8),
}