use std::str::FromStr;

#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Opcode {
    PRINT,
    MOVE,
    LOAD,
    STORE,
    ADD,
    SUB,
    MUL,
    DIV,
    AND,
    OR,
    XOR,
    SHR,
    SHL,
    JMP,
    HALT,
    CONST,
}

impl ToString for Opcode {
    fn to_string(&self) -> String {
        match self {
            Opcode::PRINT => "PRINT".to_string(),
            Opcode::MOVE => "MOVE".to_string(),
            Opcode::LOAD => "LOAD".to_string(),
            Opcode::STORE => "STORE".to_string(),
            Opcode::ADD => "ADD".to_string(),
            Opcode::SUB => "SUB".to_string(),
            Opcode::MUL => "MUL".to_string(),
            Opcode::DIV => "DIV".to_string(),
            Opcode::AND => "AND".to_string(),
            Opcode::OR => "OR".to_string(),
            Opcode::XOR => "XOR".to_string(),
            Opcode::SHR => "SHR".to_string(),
            Opcode::SHL => "SHL".to_string(),
            Opcode::JMP => "JMP".to_string(),
            Opcode::CONST => "CONST".to_string(),
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
            Opcode::SUB => 3,
            Opcode::MUL => 3,
            Opcode::DIV => 3,
            Opcode::AND => 3,
            Opcode::OR => 3,
            Opcode::XOR => 3,
            Opcode::SHR => 3,
            Opcode::SHL => 3,
            Opcode::JMP => 1,
            Opcode::CONST => 1,
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
            5 => Opcode::SUB,
            6 => Opcode::MUL,
            7 => Opcode::DIV,
            8 => Opcode::AND,
            9 => Opcode::OR,
            10 => Opcode::XOR,
            11 => Opcode::SHR,
            12 => Opcode::SHL,
            13 => Opcode::JMP,
            14 => Opcode::HALT,
            15 => Opcode::CONST,
            _ => panic!("Invalid opcode"),
        }
    }
}

impl FromStr for Opcode {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "PRINT" => Ok(Opcode::PRINT),
            "MOVE" => Ok(Opcode::MOVE),
            "LOAD" => Ok(Opcode::LOAD),
            "STORE" => Ok(Opcode::STORE),
            "ADD" => Ok(Opcode::ADD),
            "SUB" => Ok(Opcode::SUB),
            "MUL" => Ok(Opcode::MUL),
            "DIV" => Ok(Opcode::DIV),
            "AND" => Ok(Opcode::AND),
            "OR" => Ok(Opcode::OR),
            "XOR" => Ok(Opcode::XOR),
            "SHR" => Ok(Opcode::SHR),
            "SHL" => Ok(Opcode::SHL),
            "JMP" => Ok(Opcode::JMP),
            "CONST" => Ok(Opcode::CONST),
            "HALT" => Ok(Opcode::HALT),
            _ => Err(format!("Invalid opcode: {}", s)),
        }
    }
}