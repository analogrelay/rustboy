use super::{Z80Instruction, Z80RegisterName, registers, instr};
use super::mmu::Z80MMU;
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
    sp: u16,
    i: u16,
    r: u16
}

impl Z80Registers {
    fn set(&mut self, r: Z80RegisterName, val: u8) {
        match r {
            registers::A => self.a = val,
            registers::B => self.b = val,
            registers::C => self.c = val,
            registers::D => self.d = val,
            registers::E => self.e = val,
            registers::H => self.h = val,
            registers::L => self.l = val,
            registers::F => self.f = val,
            registers::I => self.i = val,
            registers::R => self.r = val,
            registers::SP => self.sp = val,
            registers::PC => self.pc = val
        }
    }

    fn get(&mut self, r: Z80RegisterName) -> u8 {
        match r {
            registers::A => self.a,
            registers::B => self.b,
            registers::C => self.c,
            registers::D => self.d,
            registers::E => self.e,
            registers::H => self.h,
            registers::L => self.l,
            registers::F => self.f,
            registers::I => self.i,
            registers::R => self.r,
            registers::SP => self.sp,
            registers::PC => self.pc
        }
    }

    fn copy_register(&mut self, r_dest: Z80RegisterName, r_src: Z80RegisterName) {
        self.set_register(r_dest, self.get_register(r_src));
    }

    fn set_pair(&mut self, d: Z80RegisterPair, val: u16) {
        if d == registers::SP {
            self.sp = val;
        } else {
            let high = ((val & 0xFF00) >> 8) as u8;
            let low = (val && 0xFF) as u8;
            match d {
                registers::BC => { self.b = high; self.c = low; },
                registers::DE => { self.d = high; self.d = low; },
                registers::HL => { self.h = high; self.l = low; },
                registers::AF => { self.a = high; self.f = low; }
                registers::SP => { self.sp = val; },
                _ => fail!("Invalid Instruction!")
            }
        }
    }

    fn get_pair(&self, d: Z80RegisterPair) -> u16 {
        if d == registers::SP {
            self.sp
        }
        else {
            let (high, low) = match d {
                registers::BC => { (self.b, self.c) },
                registers::DE => { (self.d, self.e) },
                registers::HL => { (self.h, self.l) },
                registers::AF => { (self.a, self.f) },
                _ => fail!("Invalid Instruction!")
            }
            ((high as u16) << 8) + low
        }
    }

    fn get_high(&self, d: Z80RegisterPair) -> Z80RegisterName {
        match d {
            registers::BC => registers::B,
            registers::DE => registers::D,
            registers::HL => registers::H,
            registers::AF => registers::A,
            _ => fail!("Invalid Instruction!")
        }
    }

    fn get_low(&self, d: Z80RegisterPair) -> Z80RegisterName {
        match d {
            registers::BC => registers::C,
            registers::DE => registers::E,
            registers::HL => registers::L,
            registers::AF => registers::F,
            _ => fail!("Invalid Instruction!")
        }
    }

    fn reset_flags_for_value(&mut self, v: int) {
        // Clear flags
        regs.f = 0;

        // Z flag, check if value is 0 once truncated to 255
        if (val & 0xFF) == 0 {
            regs.f |= 0x80;
        }
    }

    fn sub(&mut self, val: u8, borrow: bool) {
        // Subtract the values as full-width integers
        let val = (val as int) - (regs.a as int);

        regs.reset_flags_for_value(val);
        regs.f |= 0x40; // Subtract flag

        // Borrow flag
        if val < 0 {
            regs.f |= 0x10;
        }
    }

    fn add(&mut self, val: u8, carry: bool) {
        // Add the values as full-width integers
        let val = (val as int) + (regs.a as int);
        if carry {
            val += ((regs.f & 0x10) >> 4);
        }

        regs.reset_flags_for_value(val);

        // Carry flag
        if val > 255 {
            regs.f |= 0x10;
        }

        // Truncate back to 8-bits
        regs.a = (val & 255) as u8;
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

    pub fn exec(&mut self, i: Z80Instruction) {
        dispatch(self, i);
    }

    pub fn dump(&self) -> Z80State {
        self.state.clone()
    }

    fn tick(&mut self, m: int, t: int) {
        self.state.machine_cycles += m;
        self.state.time_cycles += t;
    }

    fn get_addr_from_hl(&self) -> u16 {
        let high = (self.state.registers.h as u16) << 8;
        high + self.state.registers.l
    }

    fn get_addr(&self, r_high: Z80RegisterName, r_low: Z80RegisterName) -> u16 {
        let high = (self.get_register(r_high) as u16) << 8;
        high + self.get_register(r_low)
    }
}

#[inline]
fn dispatch(cpu: &mut Z80, i: Z80Instruction) {
    let mut regs = cpu.state.registers;
    let mut mmu = mmu;
    match i {
        // 8-bit loads
        instr::LDrr(r1, r2) => { regs.copy(r1, r2); cpu.tick(1, 4); },
        instr::LDrHLm(r) =>    { regs.set(r, mmu.read_byte(regs.get_pair(registers::HL))); cpu.tick(2, 8); },
        instr::LDHLmr(r) =>    { mmu.write_byte(regs.get_pair(registers::HL), regs.get(r)); cpu.tick(2, 8); },
        instr::LDrn(r, n) =>   { regs.set(r, n); cpu.tick(2, 8); },
        instr::LDHLmn(n) =>    { mmu.write_byte(regs.get_pair(registers::HL), n); cpu.tick(3, 12); },
        instr::LDrrmA(r) =>    { mmu.write_byte(regs.get_pair(r), regs.a); cpu.tick(2, 8); },
        instr::LDmmA(n) =>     { mmu.write_byte(n, regs.a); cpu.tick(4, 16); },
        instr::LDArrm(r) =>    { regs.a = mmu.read_byte(regs.get_pair(r)); cpu.tick(2, 8); },
        instr::LDAmm(n) =>     { regs.a = mmu.read_byte(n); cpu.tick(4, 16); },
        instr::LDrrnn(r, n) => { regs.set_pair(r, n); cpu.tick(3, 12); },
        instr::LDHLmm(n) =>    { regs.set_pair(r, mmu.read_word(n)); cpu.tick(5, 20); },
        instr::LDmmHL(n) =>    { mmu.write_word(n, regs.get_pair(r)); cpu.tick(5, 20); },
        instr::LDHLIA => {
            mmu.write_byte(regs.get_pair(registers::HL), regs.a);
            regs.set_pair(registers::HL, regs.get_pair(registers::HL) + 1);
            cpu.tick(2, 8);
        },
        instr::LDAHLI => {
            regs.a = mmu.read_byte(regs.get_pair(registers::HL));
            regs.set_pair(registers::HL, regs.get_pair(registers::HL) + 1);
            cpu.tick(2, 8);
        }
        instr::LDHLDA => {
            mmu.write_byte(regs.get_pair(registers::HL), regs.a);
            regs.set_pair(registers::HL, regs.get_pair(registers::HL) - 1);
            cpu.tick(2, 8);
        },
        instr::LDAHLD => {
            regs.a = mmu.read_byte(regs.get_pair(registers::HL));
            regs.set_pair(registers::HL, regs.get_pair(registers::HL) - 1);
            cpu.tick(2, 8);
        },
        instr::LDAIOn(n) =>     { regs.a = mmu.read_byte(0xFF00 + n); cpu.tick(3, 12); },
        instr::LDIOnA(n) =>     { mmu.write_byte(0xFF00 + n, regs.a); cpu.tick(3, 12); },
        instr::LDAIOC    =>     { regs.a = mmu.read_byte(0xFF00 + regs.c); cpu.tick(2, 8); },
        instr::LDIOCA(n) =>     { mmu.write_byte(0xFF00 + regs.c, regs.a); cpu.tick(2, 8); },
        instr::LDHLSPn(n) =>    { regs.set_pair(registers::HL, regs.sp + n); cpu.tick(3, 12); },
        instr::SWAP(r) => {
            let t = regs.get(t);
            let up = (t & 0xF0) >> 4; // upper nybble
            let lw = t & 0x0F; // lower nybble
            regs.set(r, (lw << 4) + up); // swap and set
            cpu.tick(4, 16);
        },
        instr::ADDr(r) =>  { regs.add(regs.get(r), false); cpu.tick(1, 4); },
        instr::ADDmHL =>  { regs.add(mmu.read_byte(regs.get_pair(registers::HL)), false); cpu.tick(2, 8); },
        instr::ADDn(n) => { regs.add(n, false); cpu.tick(2, 8); },
        instr::ADDHLrr(rr) => {
            let hl = regs.get_pair(registers::HL) as int;
            hl += regs.get_pair(rr);

            // NOTE: DO NOT reset flags, Z is left alone!
            if hl > 0xFFFF {
                regs.f |= 0x10; // Set Carry
            } else {
                regs.f &= 0xEF; // Clear Carry
            }
            regs.set_pair(rr, (hl & 0xFFFF) as u16);
            cpu.tick(3, 12);
        },
        instr::ADDSPn(n) => { regs.sp += n; cpu.tick(4, 16); },
        instr::ADCr(r) =>   { regs.add(regs.get(r), true); cpu.tick(1, 4); },
        instr::ADCmHL =>    { regs.add(mmu.read_byte(regs.get_pair(registers::HL)), true); cpu.tick(2, 8); },
        instr::ADCn(n) =>   { regs.add(n, true); cpu.tick(2, 8); },
        instr::SUBr(r) =>   { regs.sub(regs.get(r), false); cpu.tick(1, 4); },
        instr::SUBmHL =>    { regs.sub(mmu.read_byte(regs.get_pair(registers::HL)), false); cpu.tick(2, 8); },
        instr::SUBn(n) =>   { regs.sub(n, false); cpu.tick(2, 8); },
        instr::SBCr(r) =>   { regs.sub(regs.get(r), true); cpu.tick(1, 4); },
        instr::SBCmHL =>    { regs.sub(mmu.read_byte(regs.get_pair(registers::HL)), true); cpu.tick(2, 8); },
        instr::SBCn(n) =>   { regs.sub(n, true); cpu.tick(2, 8); }
    }
}

#[cfg(test)]
mod test {
}