mod opcode;
mod value;
mod code;
mod machine;

use crate::code::Code;
use crate::opcode::Opcode;
use crate::value::Value;
use crate::machine::Machine;

fn main() {
    let mut machine = Machine::new();
    let mut code = Code::new();
    let val = Value::I32(10);
    let op = Opcode::LOAD;
    let op2 = Opcode::PRINT;
    let op3 = Opcode::HALT;
    code.add_const(val);
    code.write_code(op as u8, 1);
    code.write_code(0, 1);
    code.write_code(0, 1);
    code.write_code(op2 as u8, 2);
    code.write_code(0, 2);
    code.write_code(op3 as u8, 3);
    code.disassemble();
    machine.run(code);
}