#[repr(u8)]
pub enum Opcode {
    PRINT,
    MOVE,
    LOAD,
    STORE,
    ADD,
    HALT,
}

impl ToString for Opcode {
    fn to_string(&self) -> String {
        match self {
            Opcode::PRINT => "PRINT".to_string(),
            Opcode::MOVE => "MOVE".to_string(),
            Opcode::LOAD => "LOAD".to_string(),
            Opcode::STORE => "STORE".to_string(),
            Opcode::ADD => "ADD".to_string(),
            Opcode::HALT => "HALT".to_string(),
        }
    }
}

impl Opcode {
    pub fn get_offset(&self) -> usize {
        match self {
            Opcode::PRINT => 1,
            Opcode::MOVE => 2,
            Opcode::LOAD => 2,
            Opcode::STORE => 2,
            Opcode::ADD => 3,
            Opcode::HALT => 0,
        }
    }
}

impl From<u8> for Opcode {
    fn from(byte: u8) -> Self {
        match byte {
            0 => Opcode::PRINT,
            1 => Opcode::MOVE,
            2 => Opcode::LOAD,
            3 => Opcode::STORE,
            4 => Opcode::ADD,
            5 => Opcode::HALT,
            _ => panic!("Invalid opcode"),
        }
    }
}

