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

    fn adc(&self, val: u8) {
        let c = 0; // Read carry flag
        self.add(val + c);
        fail!("Carry!");
    }

    fn add(&self, val: u8) {
        let res = (self.a as int) + (val as int);

        // Reset all flags
        if(res < 0) {
            // Set S flag
        } else if(res == 0) {
            // Set Z flag
        } else if(res > 255) {
            // Set P/V flag
        }

        // Check carries
        fail!("Flags!");
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
        instr::LDrn(r, n) =>   { regs.set(r, n);    cpu.tick(2, 8); },
        instr::LDrr(r1, r2) => { regs.copy(r1, r2); cpu.tick(1, 4); },
        instr::LDrm(r) => { 
            let addr = regs.get_pair(registers::HL)
            regs.set(r, mmu.read_byte(addr)); 
            cpu.tick(2, 7); 
        },
        instr::LDmr(r) => {
            let addr = regs.get_pair(registers::HL); 
            mmu.write_byte(addr, regs.get(r)); 
            cpu.tick(2, 7); 
        },
        instr::LDmn(n) => { 
            let addr = regs.get_pair(registers::HL);
            mmu.write_byte(addr, n);
            cpu.tick(3, 10);
        },
        instr::LDadd(d) => {
            let addr = regs.get_pair(d);
            let val = mmu.read_byte(addr);
            regs.a = val;
            cpu.tick(2, 7);
        },
        instr::LDann(n) => {
            regs.set(registers::A, mmu.read_byte(n));
            cpu.tick(4, 13); 
        },
        instr::LDdda(d) => {
            let addr = regs.get_pair(d)
            mmu.write_byte(addr, regs.a);
            cpu.tick(2, 7);
        },
        instr::LDnna(n) => { 
            mmu.write_byte(n, regs.a);
            cpu.tick(4, 13); 
        },
        instr::LDai => { unimplemented!() },
        instr::LDar => { unimplemented!() },
        instr::LDia => { unimplemented!() },
        instr::LDra => { unimplemented!() },        

        // 16-bit loads
        instr::LDddnn(d, n) => { regs.set_pair(d, n);                             cpu.tick(2, 10); },
        instr::LDhlnn(n) =>    { regs.set_pair(registers::HL, mmu.read_word(n));  cpu.tick(5, 16); },
        instr::LDddm(d, n) =>  { regs.set_pair(d, mmu.read_word(n));              cpu.tick(6, 20); },
        instr::LDmhl(n) =>     { mmu.write_word(n, regs.get_pair(registers::HL)); cpu.tick(5, 16); },
        instr::LDmdd(d, n) =>  { mmu.write_word(n, regs.get_pair(d));             cpu.tick(5, 16); },
        instr::LDsphl =>       { regs.sp = regs.get_pair(registers::HL);          cpu.tick(1, 6); },
        instr::PUSHqq(q) => {
            regs.sp--;
            mmu.write_byte(regs.sp, regs.get(regs.get_high(q)));
            regs.sp--;
            mmu.write_byte(regs.sp, regs.get(regs.get_low(q)));
            cpu.tick(3, 11);
        },
        instr::POPqq(q) => {
            regs.set(regs.get_low(q), mmu.read_byte(regs.sp));
            regs.sp++;
            regs.set(regs.get_high(q), mmu.read_byte(regs.sp));
            regs.sp++;
            cpu.tick(3, 10);
        },

        // Exchange, Block Transfer, Search Group
        instr::LDI => { unimplemented!() },
        instr::LDIR => { unimplemented!() },
        instr::LDD => { unimplemented!() },
        instr::LDDR => { unimplemented!() },
        instr::CPI => { unimplemented!() },
        instr::CPIR => { unimplemented!() },
        instr::CPD => { unimplemented!() },
        instr::CPDR => { unimplemented!() },

        // 8-bit arithmetic
        instr::ADDar(r) => { regs.adc(regs.get(r));                                 cpu.tick(1, 4); }
        instr::ADDan(n) => { regs.add(n);                                           cpu.tick(2, 7); }
        instr::ADDahl =>   { regs.add(mmu.read_byte(regs.get_pair(registers::HL))); cpu.tick(2, 7); }
        instr::ADCar(r) => { regs.adc(regs.get(r));                                 cpu.tick(1, 4); }
        instr::ADCan(n) => { regs.adc(n);                                           cpu.tick(2, 7); }
        instr::ADCahl =>   { regs.adc(mmu.read_byte(regs.get_pair(registers::HL))); cpu.tick(2, 7); }
           
    }
}