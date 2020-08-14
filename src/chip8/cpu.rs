use super::rng::RNG;
use super::mem::Memory;
use super::dsp::Display;
use super::kbd::Keyboard;
use super::op::Op;
use super::set::Settings;

const STACK_SIZE: usize = 16;

pub struct CPU {
    v: [u8; 16],
    i: u16,
    pc: u16,
    sp: u8,
    stack: [u16; STACK_SIZE],
    dt: u8,
    st: u8,
}

impl CPU {
    pub fn new() -> Self {
        Self {
            v: [0; 16],
            i: 0,
            pc: 0x200,
            sp: 0,
            stack: [0; STACK_SIZE],
            dt: 0,
            st: 0
        }
    }

    pub fn cycle_dt(&mut self) {
        if self.dt > 0 {
            self.dt -= 1;
        }
    }

    pub fn cycle_st(&mut self) -> bool {
        if self.st > 0 {
            self.st -= 1;
        }
        self.st > 0
    }

    pub fn cycle(
        &mut self,
        memory: &mut Memory,
        display: &mut Display,
        keyboard: &mut Keyboard,
        rng: &mut RNG,
        settings: &Settings,
    ) {
        let opcode = ((memory.read(self.pc) as u16) << 8) | (memory.read(self.pc + 1) as u16);
        let opaddr = self.pc;
        self.pc += 2;

        let op = Op::decode(opcode);

        if settings.print_opcodes {
            println!("{:#05X}: {:#06X} {}", opaddr, opcode, op);
        }

        match op {
            Op::SYS  { addr }                 => self.sys(addr),
            Op::CLS                           => self.cls(display),
            Op::RET                           => self.ret(),
            Op::JP   { addr }                 => self.jp(addr),
            Op::CALL { addr }                 => self.call(addr),
            Op::SE   { reg, byte }            => self.se(reg, byte),
            Op::SNE  { reg, byte }            => self.sne(reg, byte),
            Op::SER  { reg_a, reg_b }         => self.ser(reg_a, reg_b),
            Op::LD   { reg, byte }            => self.ld(reg, byte),
            Op::ADD  { reg, byte }            => self.add(reg, byte),
            Op::LDR  { reg_a, reg_b }         => self.ldr(reg_a, reg_b),
            Op::OR   { reg_a, reg_b }         => self.or(reg_a, reg_b),
            Op::AND  { reg_a, reg_b }         => self.and(reg_a, reg_b),
            Op::XOR  { reg_a, reg_b }         => self.xor(reg_a, reg_b),
            Op::ADDR { reg_a, reg_b }         => self.addr(reg_a, reg_b),
            Op::SUB  { reg_a, reg_b }         => self.sub(reg_a, reg_b),
            Op::SHR  { reg_a, reg_b }         => self.shr(reg_a, reg_b, settings),
            Op::SUBN { reg_a, reg_b }         => self.subn(reg_a, reg_b),
            Op::SHL  { reg_a, reg_b }         => self.shl(reg_a, reg_b, settings),
            Op::SNER { reg_a, reg_b }         => self.sner(reg_a, reg_b),
            Op::LDA  { addr }                 => self.lda(addr),
            Op::JPO  { addr }                 => self.jpo(addr),
            Op::RND  { reg, byte }            => self.rnd(reg, byte, rng),
            Op::DRW  { reg_a, reg_b, nibble } => self.drw(reg_a, reg_b, nibble, memory, display, settings),
            Op::SKP  { reg }                  => self.skp(reg, keyboard),
            Op::SKNP { reg }                  => self.sknp(reg, keyboard),
            Op::LDDT { reg }                  => self.lddt(reg),
            Op::LDKP { reg }                  => self.ldkp(reg, keyboard),
            Op::STDT { reg }                  => self.stdt(reg),
            Op::STST { reg }                  => self.stst(reg),
            Op::ADDA { reg }                  => self.adda(reg, settings),
            Op::LDSA { reg }                  => self.ldsa(reg),
            Op::STDR { reg }                  => self.stdr(reg, memory),
            Op::STRR { reg }                  => self.strr(reg, memory, settings),
            Op::LDRR { reg }                  => self.ldrr(reg, memory, settings),
            Op::INV  { opcode }               => panic!("Invalid opcode {:#06X} at {:#05X}", opcode, opaddr),
        }
    }

    // Ops

    fn sys(&mut self, _addr: u16) {
        // NOOP
    }

    fn cls(&mut self, display: &mut Display) {
        display.clear();
        //println!("{}", display);
    }

    fn ret(&mut self) {
        self.sp -= 1;
        self.pc = self.stack[self.sp as usize];
    }

    fn jp(&mut self, addr: u16) {
        self.pc = addr;
    }

    fn call(&mut self, addr: u16) {
        self.stack[self.sp as usize] = self.pc;
        self.sp += 1;
        self.pc = addr;
    }

    fn se(&mut self, reg: u8, byte: u8) {
        if self.v[reg as usize] == byte {
            self.pc += 2;
        }
    }

    fn sne(&mut self, reg: u8, byte: u8) {
        if self.v[reg as usize] != byte {
            self.pc += 2;
        }
    }

    fn ser(&mut self, reg_a: u8, reg_b: u8) {
        if self.v[reg_a as usize] == self.v[reg_b as usize] {
            self.pc += 2;
        }
    }

    fn ld(&mut self, reg: u8, byte: u8) {
        self.v[reg as usize] = byte;
    }

    fn add(&mut self, reg: u8, byte: u8) {
        let (sum, _) = self.v[reg as usize].overflowing_add(byte);
        self.v[reg as usize] = sum;
    }

    fn ldr(&mut self, reg_a: u8, reg_b: u8) {
        self.v[reg_a as usize] = self.v[reg_b as usize];
    }

    fn or(&mut self, reg_a: u8, reg_b: u8) {
        self.v[reg_a as usize] |= self.v[reg_b as usize];
    }

    fn and(&mut self, reg_a: u8, reg_b: u8) {
        self.v[reg_a as usize] &= self.v[reg_b as usize];
    }

    fn xor(&mut self, reg_a: u8, reg_b: u8) {
        self.v[reg_a as usize] ^= self.v[reg_b as usize];
    }

    fn addr(&mut self, reg_a: u8, reg_b: u8) {
        let (sum, carry) = self.v[reg_a as usize].overflowing_add(self.v[reg_b as usize]);
        self.v[reg_a as usize] = sum;
        self.v[0xF] = if carry { 1 } else { 0 };
    }

    fn sub(&mut self, reg_a: u8, reg_b: u8) {
        let (dif, borrow) = self.v[reg_a as usize].overflowing_sub(self.v[reg_b as usize]);
        self.v[reg_a as usize] = dif;
        self.v[0xF] = if borrow { 0 } else { 1 };
    }

    fn shr(&mut self, reg_a: u8, mut reg_b: u8, settings: &Settings) {
        if settings.shift_quirk {
            reg_b = reg_a;
        }
        let lsb = self.v[reg_b as usize] & 1;
        self.v[reg_a as usize] = self.v[reg_b as usize] >> 1;
        self.v[0xF] = lsb;
    }

    fn subn(&mut self, reg_a: u8, reg_b: u8) {
        let (dif, borrow) = self.v[reg_b as usize].overflowing_sub(self.v[reg_a as usize]);
        self.v[reg_a as usize] = dif;
        self.v[0xF] = if borrow { 0 } else { 1 };
    }

    fn shl(&mut self, reg_a: u8, mut reg_b: u8, settings: &Settings) {
        if settings.shift_quirk {
            reg_b = reg_a;
        }
        let msb = (self.v[reg_b as usize] >> 7) & 1;
        self.v[reg_a as usize] = self.v[reg_b as usize] << 1;
        self.v[0xF] = msb;
    }

    fn sner(&mut self, reg_a: u8, reg_b: u8) {
        if self.v[reg_a as usize] != self.v[reg_b as usize] {
            self.pc += 2;
        }
    }

    fn lda(&mut self, addr: u16) {
        self.i = addr;
    }

    fn jpo(&mut self, addr: u16) {
        self.pc = addr + self.v[0] as u16;
    }

    fn rnd(&mut self, reg: u8, byte: u8, rng: &mut RNG) {
        self.v[reg as usize] = rng.next() & byte;
    }

    fn drw(&mut self, reg_a: u8, reg_b: u8, nibble: u8, memory: &mut Memory, display: &mut Display, settings: &Settings) {
        let x = self.v[reg_a as usize] as usize;
        let y = self.v[reg_b as usize] as usize;
        let h = nibble as usize;

        let mut pixel_erased = false;

        for offset_y in 0..h {
            let px_y = y + offset_y;
            if !settings.vertical_wrap && px_y >= display.height() {
                break;
            }
            let byte = memory.read(self.i + offset_y as u16);
            for offset_x in 0..8 {
                let pixel = ((byte >> 7 - offset_x) & 1) == 1;
                let px_x = x + offset_x;
                pixel_erased |= display.set_pixel(px_x, px_y, pixel);
            }
        }

        self.v[0xF] = if pixel_erased { 1 } else { 0 };

        //println!("{}", display);
    }

    fn skp(&mut self, reg: u8, keyboard: &mut Keyboard) {
        if keyboard.get_key(self.v[reg as usize]) {
            self.pc += 2;
        }
    }

    fn sknp(&mut self, reg: u8, keyboard: &mut Keyboard) {
        if !keyboard.get_key(self.v[reg as usize]) {
            self.pc += 2;
        }
    }

    fn lddt(&mut self, reg: u8) {
        self.v[reg as usize] = self.dt;
    }

    fn ldkp(&mut self, reg: u8, keyboard: &mut Keyboard) {
        if let Some(key) = keyboard.wait_keypress() {
            self.v[reg as usize] = key;
        } else {
            self.pc -= 2;
        }
    }

    fn stdt(&mut self, reg: u8) {
        self.dt = self.v[reg as usize];
    }

    fn stst(&mut self, reg: u8) {
        self.st = self.v[reg as usize];
    }

    fn adda(&mut self, reg: u8, settings: &Settings) {
        let (addr, carry) = self.i.overflowing_add(self.v[reg as usize] as u16);
        self.i = addr;
        if settings.address_overflow_quirk {
            self.v[0xF] = if carry { 1 } else { 0 };
        }
    }

    fn ldsa(&mut self, reg: u8) {
        self.i = Memory::sprite_address(self.v[reg as usize]);
    }

    fn stdr(&mut self, reg: u8, memory: &mut Memory) {
        memory.write(self.i,      self.v[reg as usize] / 100);
        memory.write(self.i + 1, (self.v[reg as usize] / 10) % 10);
        memory.write(self.i + 2,  self.v[reg as usize] % 10)
    }

    fn strr(&mut self, reg: u8, memory: &mut Memory, settings: &Settings) {
        for i in 0..=reg {
            let addr = self.i + i as u16;
            memory.write(addr, self.v[i as usize]);
        }
        if settings.load_store_quirk {
            self.i += reg as u16 + 1;
        }
    }

    fn ldrr(&mut self, reg: u8, memory: &mut Memory, settings: &Settings) {
        for i in 0..=reg {
            let addr = self.i + i as u16;
            self.v[i as usize] = memory.read(addr);
        }
        if settings.load_store_quirk {
            self.i += reg as u16 + 1;
        }
    }
}
