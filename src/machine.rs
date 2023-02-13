use crate::{code::Code, value::Value};

const REGISTER_MAX: usize = u8::MAX as usize;

pub struct Machine {
    registers: [Value; REGISTER_MAX],
    pc: usize,
    code: Code,
}

impl Machine {
    pub fn new() -> Machine {
        Machine {
            registers: [Value::U8(0); REGISTER_MAX],
            pc: 0,
            code: Code::new(),
        }
    }

    fn print(&self, reg: u8) {
        print!("{}", self.registers[reg as usize]);
    }

    fn move_reg(&mut self, r1: u8, r2: u8) {
        self.registers[r1 as usize] = self.registers[r2 as usize];

    }

    fn add(&mut self, r1: u8, r2: u8, r3: u8) {
        self.registers[r1 as usize] = self.registers[r2 as usize] + self.registers[r3 as usize];
    }

    fn load(&mut self, reg: u8, constant: u8) {
        self.registers[reg as usize] = self.code.const_pool[constant as usize];
    }

    fn store(&mut self, reg: u8, constant: u8) {
        self.code.const_pool[constant as usize] = self.registers[reg as usize];
    }

    pub fn run(&mut self, code: Code) {
        self.code = code;
        loop {
            let instruction = self.code.raw[self.pc];
            match instruction {
                0 => {
                    self.print(self.code.raw[self.pc + 1]);
                    self.pc += 2;
                }
                1 => {
                    self.move_reg(self.code.raw[self.pc + 1], self.code.raw[self.pc + 2]);
                    self.pc += 3;
                }
                2 => {
                    self.load(self.code.raw[self.pc + 1], self.code.raw[self.pc + 2]);
                    self.pc += 3;
                }
                3 => {
                    self.store(self.code.raw[self.pc + 1], self.code.raw[self.pc + 2]);
                    self.pc += 3;
                }
                4 => {
                    self.add(self.code.raw[self.pc + 1], self.code.raw[self.pc + 2], self.code.raw[self.pc + 3]);
                    self.pc += 4;
                }
                5 => {
                    break;
                }
                _ => {
                    panic!("Invalid opcode");
                }
            }
        }
    }

}

