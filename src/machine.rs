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

    fn sub(&mut self, r1: u8, r2: u8, r3: u8) {
        self.registers[r1 as usize] = self.registers[r2 as usize] - self.registers[r3 as usize];
    }

    fn mul(&mut self, r1: u8, r2: u8, r3: u8) {
        self.registers[r1 as usize] = self.registers[r2 as usize] * self.registers[r3 as usize];
    }

    fn div(&mut self, r1: u8, r2: u8, r3: u8) {
        self.registers[r1 as usize] = self.registers[r2 as usize] / self.registers[r3 as usize];
    }

    fn and(&mut self, r1: u8, r2: u8, r3: u8) {
        self.registers[r1 as usize] = self.registers[r2 as usize] & self.registers[r3 as usize];
    }

    fn or(&mut self, r1: u8, r2: u8, r3: u8) {
        self.registers[r1 as usize] = self.registers[r2 as usize] | self.registers[r3 as usize];
    }

    fn xor(&mut self, r1: u8, r2: u8, r3: u8) {
        self.registers[r1 as usize] = self.registers[r2 as usize] ^ self.registers[r3 as usize];
    }

    fn shr(&mut self, r1: u8, r2: u8, r3: u8) {
        self.registers[r1 as usize] = self.registers[r2 as usize] >> self.registers[r3 as usize];
    }

    fn shl(&mut self, r1: u8, r2: u8, r3: u8) {
        self.registers[r1 as usize] = self.registers[r2 as usize] << self.registers[r3 as usize];
    }

    fn load(&mut self, reg: u8, constant: u8) {
        self.registers[reg as usize] = self.code.const_pool[constant as usize];
    }

    fn store(&mut self, reg: u8, constant: u8) {
        self.code.const_pool[constant as usize] = self.registers[reg as usize];
    }

    fn jmp(&mut self, addr: u8) {
        self.pc = addr as usize;
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
                    self.sub(self.code.raw[self.pc + 1], self.code.raw[self.pc + 2], self.code.raw[self.pc + 3]);
                    self.pc += 4;
                }
                6 => {
                    self.mul(self.code.raw[self.pc + 1], self.code.raw[self.pc + 2], self.code.raw[self.pc + 3]);
                    self.pc += 4;
                }
                7 => {
                    self.div(self.code.raw[self.pc + 1], self.code.raw[self.pc + 2], self.code.raw[self.pc + 3]);
                    self.pc += 4;
                }
                8 => {
                    self.and(self.code.raw[self.pc + 1], self.code.raw[self.pc + 2], self.code.raw[self.pc + 3]);
                    self.pc += 4;
                }
                9 => {
                    self.or(self.code.raw[self.pc + 1], self.code.raw[self.pc + 2], self.code.raw[self.pc + 3]);
                    self.pc += 4;
                }
                10 => {
                    self.xor(self.code.raw[self.pc + 1], self.code.raw[self.pc + 2], self.code.raw[self.pc + 3]);
                    self.pc += 4;
                }
                11 => {
                    self.shr(self.code.raw[self.pc + 1], self.code.raw[self.pc + 2], self.code.raw[self.pc + 3]);
                    self.pc += 4;
                }
                12 => {
                    self.shl(self.code.raw[self.pc + 1], self.code.raw[self.pc + 2], self.code.raw[self.pc + 3]);
                    self.pc += 4;
                }
                13 => {
                    self.jmp(self.code.raw[self.pc + 1]);
                }
                14 => {
                    break;
                }
                _ => {
                    panic!("Invalid opcode {:?} at {}", instruction, self.pc);
                }
            }
        }
    }

}

// tests for machine
#[cfg(test)]
mod tests {
    use super::*;
    use crate::opcode::Opcode;

    #[test]
    fn test_load() {
        let mut machine = Machine::new();
        let mut code = Code::new();
        code.add_const(Value::from(1i8));
        code.write_code(Opcode::LOAD as u8, 0);
        code.write_code(0, 0);
        code.write_code(0, 0);   
        code.write_code(Opcode::HALT as u8, 1); 
        machine.run(code);
        assert_eq!(machine.registers[0], Value::I8(1));
    }

    #[test]
    fn test_store() {
        let mut machine = Machine::new();
        let mut code = Code::new();
        code.add_const(Value::from(1i8));
        code.write_code(Opcode::LOAD as u8, 0);
        code.write_code(0, 0);
        code.write_code(0, 0);   
        code.write_code(Opcode::STORE as u8, 1); 
        code.write_code(0, 1);
        code.write_code(0, 1);   
        code.write_code(Opcode::HALT as u8, 2); 
        machine.run(code);
        assert_eq!(machine.code.const_pool[0], Value::I8(1));
    }

    #[test]
    fn test_move() {
        let mut machine = Machine::new();
        let mut code = Code::new();
        code.add_const(Value::from(10i8));
        code.write_code(Opcode::LOAD as u8, 0);
        code.write_code(0, 0);
        code.write_code(0, 0);   
        code.write_code(Opcode::MOVE as u8, 1); 
        code.write_code(1, 1);
        code.write_code(0, 1);   
        code.write_code(Opcode::HALT as u8, 2); 
        machine.run(code);
        assert_eq!(machine.registers[1], Value::I8(10));
    }

    #[test]
    fn test_add() {
        let mut machine = Machine::new();
        let mut code = Code::new();
        code.add_const(Value::from(10i8));
        code.add_const(Value::from(20i8));
        code.write_code(Opcode::LOAD as u8, 0);
        code.write_code(0, 0);
        code.write_code(0, 0);
        code.write_code(Opcode::LOAD as u8, 1);
        code.write_code(1, 1);
        code.write_code(1, 1);
        code.write_code(Opcode::ADD as u8, 2);
        code.write_code(2, 2);
        code.write_code(0, 2);
        code.write_code(1, 2);
        code.write_code(Opcode::HALT as u8, 3);
        machine.run(code);
        assert_eq!(machine.registers[2], Value::I8(30));
    }
}