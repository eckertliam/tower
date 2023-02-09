use std::fmt::Debug;

use crate::{machine::Machine, constant::Constant};

pub type InstructionFn = fn(machine: &mut Machine, args: &[Constant]);

#[derive(Clone)]
pub struct Instruction {
    pub op_code: u8,
    pub name: String,
    pub arity: u8,
    pub func: InstructionFn,
}

impl Debug for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Instruction: {}\nOpcode: {}\nArity: {}", self.name, self.op_code, self.arity)
    }
}

impl Instruction {
    pub fn new(op_code: u8, n_str: &str, arity: u8, func: InstructionFn) -> Self {
        Self {
            op_code,
            name: n_str.to_string(),
            arity,
            func,
        }
    }

    pub fn call(self) -> InstructionFn {
        self.func
    }
}



#[cfg(test)]
mod test {
    use super::*;

    fn int_add(machine: &mut Machine, args: &[Constant]) {
        let lhs: i64 = machine.pop().into();
        let rhs: i64 = machine.pop().into();
        machine.push(Constant::from(lhs + rhs));
    }

    #[test]
    fn fn_call() {
        let mut machine = Machine::new();
        machine.push(Constant::from(1));
        machine.push(Constant::from(2));
        let instr: Instruction = Instruction::new(0, "add", 0, int_add);
        instr.call()(&mut machine, &[]);
        let result: i64 = machine.pop().into();
        assert_eq!(3, result);
    }
}