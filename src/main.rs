mod opcode;
mod value;
mod code;
mod machine;
mod reader;

use crate::code::Code;
use crate::opcode::Opcode;
use crate::value::Value;
use crate::machine::Machine;

fn main() {
    let mut machine = Machine::new();
    let mut code = Code::new();
    let val = Value::F32(10.5);
    let val3 = Value::F32(30.5);
    code.add_const(val);
    code.add_const(val3);
    code.write_code(Opcode::LOAD as u8, 1);
    code.write_code(0, 1);
    code.write_code(0, 1);
    code.write_code(Opcode::LOAD as u8, 2);
    code.write_code(1, 2);
    code.write_code(1, 2);
    code.write_code(Opcode::ADD as u8, 3);
    code.write_code(2, 3);
    code.write_code(0, 3);
    code.write_code(1, 3);
    code.write_code(Opcode::PRINT as u8, 4);
    code.write_code(2, 4);
    code.write_code(Opcode::HALT as u8, 4);
    code.disassemble();
    machine.run(code);
}