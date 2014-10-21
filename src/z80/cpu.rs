use super::{Z80Instruction, Z80RegisterName, Z80RegisterPair, registers, instr};
use super::mmu::Z80MMU;
use std::fmt;

#[deriving(Clone)]
pub struct Z80State {
    machine_cycles: int,
    time_cycles: int,
    registers: Z80Registers
}

impl Z80State {
    fn tick(&mut self, m: int, t: int) {
        self.machine_cycles += m;
        self.time_cycles += t;
    }
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
    sp: u16,
    i: u16,
    r: u16
}

impl Z80Registers {
    fn set(&mut self, r: Z80RegisterName, val: u8) {
        debug!("register {} <- {}", r, val);
        match r {
            registers::A => self.a = val,
            registers::B => self.b = val,
            registers::C => self.c = val,
            registers::D => self.d = val,
            registers::E => self.e = val,
            registers::H => self.h = val,
            registers::L => self.l = val,
            registers::F => self.f = val,
            _ => fail!("Cannot set {} register using this method", r)
        }
    }

    fn get(&mut self, r: Z80RegisterName) -> u8 {
        debug!("register {} ->", r);
        match r {
            registers::A => self.a,
            registers::B => self.b,
            registers::C => self.c,
            registers::D => self.d,
            registers::E => self.e,
            registers::H => self.h,
            registers::L => self.l,
            registers::F => self.f,
            _ => fail!("Cannot get {} register using this method", r)
        }
    }

    fn copy(&mut self, r_dest: Z80RegisterName, r_src: Z80RegisterName) {
        let val = self.get(r_src);
        self.set(r_dest, val);
    }

    fn set_pair(&mut self, d: Z80RegisterPair, val: u16) {
        debug!("registers {} <- {}", d, val);
        if d == registers::SP_ {
            self.sp = val;
        } else {
            let high = ((val & 0xFF00) >> 8) as u8;
            let low = (val & 0xFF) as u8;
            match d {
                registers::BC => { self.b = high; self.c = low; },
                registers::DE => { self.d = high; self.d = low; },
                registers::HL => { self.h = high; self.l = low; },
                registers::AF => { self.a = high; self.f = low; }
                _ => fail!("Invalid Instruction!")
            }
        }
    }

    fn get_pair(&self, d: Z80RegisterPair) -> u16 {
        debug!("registers {} ->", d);
        if d == registers::SP_ {
            self.sp
        }
        else {
            let (high, low) = match d {
                registers::BC => { (self.b, self.c) },
                registers::DE => { (self.d, self.e) },
                registers::HL => { (self.h, self.l) },
                registers::AF => { (self.a, self.f) },
                _ => fail!("Invalid Instruction!")
            };
            ((high as u16) << 8) + (low as u16)
        }
    }

    fn reset_flags_for_value(&mut self, val: int) {
        // Clear flags
        self.f = 0;

        // Z flag, check if value is 0 once truncated to 255
        if (val & 0xFF) == 0 {
            self.f |= 0x80;
        }
    }

    fn sub(&mut self, val: u8, borrow: bool) {
        // Subtract the values as full-width integers
        let mut val = (val as int) - (self.a as int);
        if borrow {
            val -= ((self.f as int) & 0x10) >> 4;
        }

        self.reset_flags_for_value(val);
        self.f |= 0x40; // Subtract flag

        // Borrow flag
        if val < 0 {
            self.f |= 0x10;
        }

        // Truncate back to 8-bits
        self.a = (val & 255) as u8;
    }

    fn add(&mut self, val: u8, carry: bool) {
        // Add the values as full-width integers
        let mut val = (val as int) + (self.a as int);
        if carry {
            val += ((self.f as int) & 0x10) >> 4;
        }

        self.reset_flags_for_value(val);

        // Carry flag
        if val > 255 {
            self.f |= 0x10;
        }

        // Truncate back to 8-bits
        self.a = (val & 255) as u8;
    }
}

pub struct Z80 {
    state: Z80State,
    mmu: Z80MMU
}

impl fmt::Show for Z80 {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "Z80 a={a:04} b={b} c={c} d={d} e={e} h={h} l={l} f={f} pc={pc} sp={sp} m={m} t={t} i={i} r={r}",
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
            i = self.state.registers.i,
            r = self.state.registers.r,
            m = self.state.machine_cycles,
            t = self.state.time_cycles)
    }
}

impl Z80 {
    pub fn new() -> Z80 {
        Z80 {
            mmu: Z80MMU::new(),
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
                    sp: 0,
                    i: 0,
                    r: 0
                }
            }
        }
    }

    pub fn state(&self) -> &Z80State {
        &(self.state)
    }

    pub fn regs(&self) -> &Z80Registers {
        &(self.state.registers)
    }

    pub fn exec(&mut self, i: Z80Instruction) {
        dispatch(self, i);
    }

    pub fn dump(&self) -> Z80State {
        self.state.clone()
    }
}

#[inline]
fn dispatch(cpu: &mut Z80, i: Z80Instruction) {
    let state = &mut cpu.state;
    let mmu = &mut cpu.mmu;
    match i {
        // 8-bit loads
        instr::LDrr(r1, r2) => { state.registers.copy(r1, r2); state.tick(1, 4); },
        instr::LDrHLm(r) =>    { let hl = state.registers.get_pair(registers::HL); state.registers.set(r, mmu.read_byte(hl)); state.tick(2, 8); },
        instr::LDHLmr(r) =>    { let hl = state.registers.get_pair(registers::HL); mmu.write_byte(hl, state.registers.get(r)); state.tick(2, 8); },
        instr::LDrn(r, n) =>   { state.registers.set(r, n); state.tick(2, 8); },
        instr::LDHLmn(n) =>    { mmu.write_byte(state.registers.get_pair(registers::HL), n); state.tick(3, 12); },
        instr::LDrrmA(r) =>    { mmu.write_byte(state.registers.get_pair(r), state.registers.a); state.tick(2, 8); },
        instr::LDmmA(n) =>     { mmu.write_byte(n, state.registers.a); state.tick(4, 16); },
        instr::LDArrm(r) =>    { state.registers.a = mmu.read_byte(state.registers.get_pair(r)); state.tick(2, 8); },
        instr::LDAmm(n) =>     { state.registers.a = mmu.read_byte(n); state.tick(4, 16); },
        instr::LDrrnn(r, n) => { state.registers.set_pair(r, n); state.tick(3, 12); },
        instr::LDHLmm(n) =>    { state.registers.set_pair(registers::HL, mmu.read_word(n)); state.tick(5, 20); },
        instr::LDmmHL(n) =>    { mmu.write_word(n, state.registers.get_pair(registers::HL)); state.tick(5, 20); },
        instr::LDHLIA => {
            let hl = state.registers.get_pair(registers::HL);
            mmu.write_byte(hl, state.registers.a);
            state.registers.set_pair(registers::HL, hl + 1);
            state.tick(2, 8);
        },
        instr::LDAHLI => {
            let hl = state.registers.get_pair(registers::HL);
            state.registers.a = mmu.read_byte(hl);
            state.registers.set_pair(registers::HL, hl + 1);
            state.tick(2, 8);
        }
        instr::LDHLDA => {
            let hl = state.registers.get_pair(registers::HL);
            mmu.write_byte(hl, state.registers.a);
            state.registers.set_pair(registers::HL, hl - 1);
            state.tick(2, 8);
        },
        instr::LDAHLD => {
            let hl = state.registers.get_pair(registers::HL);
            state.registers.a = mmu.read_byte(hl);
            state.registers.set_pair(registers::HL, hl - 1);
            state.tick(2, 8);
        },
        instr::LDAIOn(n) =>     { state.registers.a = mmu.read_byte(0xFF00 + (n as u16)); state.tick(3, 12); },
        instr::LDIOnA(n) =>     { mmu.write_byte(0xFF00 + (n as u16), state.registers.a); state.tick(3, 12); },
        instr::LDAIOC    =>     { state.registers.a = mmu.read_byte(0xFF00 + (state.registers.c as u16)); state.tick(2, 8); },
        instr::LDIOCA    =>     { mmu.write_byte(0xFF00 + (state.registers.c as u16), state.registers.a); state.tick(2, 8); },
        instr::LDHLSPn(n) =>    { let val = state.registers.sp + (n as u16); state.registers.set_pair(registers::HL, val); state.tick(3, 12); },
        instr::SWAP(r) => {
            let t = state.registers.get(r);
            let up = (t & 0xF0) >> 4; // upper nybble
            let lw = t & 0x0F; // lower nybble
            state.registers.set(r, (lw << 4) + up); // swap and set
            state.tick(4, 16);
        },
        instr::ADDr(r) =>  { let v = state.registers.get(r); state.registers.add(v, false); state.tick(1, 4); },
        instr::ADDmHL =>  { let hl = state.registers.get_pair(registers::HL); state.registers.add(mmu.read_byte(hl), false); state.tick(2, 8); },
        instr::ADDn(n) => { state.registers.add(n, false); state.tick(2, 8); },
        instr::ADDHLrr(rr) => {
            let mut hl = state.registers.get_pair(registers::HL) as int;
            hl += state.registers.get_pair(rr) as int;

            // NOTE: DO NOT reset flags, Z is left alone!
            if hl > 0xFFFF {
                state.registers.f |= 0x10; // Set Carry
            } else {
                state.registers.f &= 0xEF; // Clear Carry
            }
            state.registers.set_pair(rr, (hl & 0xFFFF) as u16);
            state.tick(3, 12);
        },
        instr::ADDSPn(n) => { state.registers.sp = ((n as int) + (state.registers.sp as int)) as u16; state.tick(4, 16); },
        instr::ADCr(r) =>   { let v = state.registers.get(r); state.registers.add(v, true); state.tick(1, 4); },
        instr::ADCmHL =>    { let hl = state.registers.get_pair(registers::HL); state.registers.add(mmu.read_byte(hl), true); state.tick(2, 8); },
        instr::ADCn(n) =>   { state.registers.add(n, true); state.tick(2, 8); },
        instr::SUBr(r) =>   { let v = state.registers.get(r); state.registers.sub(v, false); state.tick(1, 4); },
        instr::SUBmHL =>    { let hl = state.registers.get_pair(registers::HL); state.registers.sub(mmu.read_byte(hl), false); state.tick(2, 8); },
        instr::SUBn(n) =>   { state.registers.sub(n, false); state.tick(2, 8); },
        instr::SBCr(r) =>   { let v = state.registers.get(r); state.registers.sub(v, true); state.tick(1, 4); },
        instr::SBCmHL =>    { let hl = state.registers.get_pair(registers::HL); state.registers.sub(mmu.read_byte(hl), true); state.tick(2, 8); },
        instr::SBCn(n) =>   { state.registers.sub(n, true); state.tick(2, 8); }
    }
}

#[cfg(test)]
mod test {
    use z80::{Z80, registers, instr};

    macro_rules! testcpu(
        ($(on $i:expr, $assert_l:expr is $assert_r: expr ticks $m:expr, $t:expr),+) => ({
            let mut cpu = Z80::new();
            $(
                cpu.exec($i);
                assert_eq!($assert_l, $assert_r);
                assert_eq!(cpu.state.machine_cycles, $m);
                assert_eq!(cpu.state.time_cycles, $t);
            )+
        });
    )

    #[test]
    pub fn test_ld_r_n() {
        testcpu!(
            on instr::LDrn(registers::B, 42), cpu.regs().b is 42 ticks 2, 8,
            on instr::LDrn(registers::C, 42), cpu.regs().c is 42 ticks 2, 8,
            on instr::LDrn(registers::D, 42), cpu.regs().d is 42 ticks 2, 8,
            on instr::LDrn(registers::E, 42), cpu.regs().e is 42 ticks 2, 8,
            on instr::LDrn(registers::H, 42), cpu.regs().h is 42 ticks 2, 8,
            on instr::LDrn(registers::L, 42), cpu.regs().l is 42 ticks 2, 8
        )
    }
}