use crate::{value::Value, opcode::Opcode};

pub struct Code {
    pub raw: Vec<u8>,
    pub lines: Vec<usize>,
    pub const_pool: Vec<Value>,
}

impl Code {
    pub fn new() -> Code {
        Code {
            raw: Vec::new(),
            lines: Vec::new(),
            const_pool: Vec::new(),
        }
    }

    pub fn add_const(&mut self, value: Value) -> usize {
        self.const_pool.push(value);
        self.const_pool.len() - 1
    }

    pub fn write_code(&mut self, code: u8, line: usize) {
        self.raw.push(code);
        self.lines.push(line);
    }

    fn disassemble_instruction(&self, offset: usize) -> usize {
        print!("{}", offset);
        print!(" {} ", self.lines[offset]);
        let instruction: Opcode = self.raw[offset].into();

        match instruction {
            Opcode::PRINT => {
                let register = self.raw[offset + 1];
                println!("PRINT ${}", register);
                offset + instruction.get_offset() + 1
            }
            Opcode::MOVE => {
                let r1 = self.raw[offset + 1];
                let r2 = self.raw[offset + 2];
                println!("MOVE ${} ${}", r1, r2);
                offset + instruction.get_offset() + 1
            }
            Opcode::LOAD => {
                let register = self.raw[offset + 1];
                let constant = self.raw[offset + 2];
                println!("LOAD ${} {}", register, self.const_pool[constant as usize]);
                offset + instruction.get_offset() + 1
            }
            Opcode::STORE => {
                let register = self.raw[offset + 1];
                let constant = self.raw[offset + 2];
                println!("STORE ${} {}", register, self.const_pool[constant as usize]);
                offset + instruction.get_offset() + 1
            }
            Opcode::ADD => {
                let r1 = self.raw[offset + 1];
                let r2 = self.raw[offset + 2];
                let r3 = self.raw[offset + 3];
                println!("ADD ${} ${} ${}", r1, r2, r3);
                offset + instruction.get_offset() + 1
            }
            Opcode::HALT => {
                println!("HALT");
                offset + 1
            }
        }
    }

    pub fn disassemble(&self) {
        let mut offset = 0;
        while offset < self.raw.len() {
            offset = self.disassemble_instruction(offset);
        }
    }
}