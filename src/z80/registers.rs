#[deriving(PartialEq, Eq, Show)]
pub enum Z80RegisterName {
	A,
	B,
	C,
	D,
	E,
	H,
	L,
	F,
	SP,
	PC
}

#[deriving(PartialEq, Eq, Show)]
pub enum Z80RegisterPair {
	BC,
	DE,
	HL,
	AF,
	SP_
}