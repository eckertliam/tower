use crate::instruction::Instruction;
use crate::ops::*;

use std::collections::HashMap;

// A hashmap of instructions
#[derive(Debug, Clone)]
pub struct InstructionTable(HashMap<u8, Instruction>);

impl InstructionTable {
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    // Add an instruction to the table
    pub fn push(&mut self, instr: Instruction) {
        match self.0.get(&instr.op_code) {
            Some(conflict) => {
                panic!("Error: op code conflict between {:?} and {:?}", conflict, instr)
            }
            None => self.0.insert(instr.op_code, instr),
        };
    }

    // Return a hashmap of the instruction names and their op codes
    pub fn symbol_table(self) -> HashMap<String, u8> {
        let mut table: HashMap<String, u8> = HashMap::new();
        self.0.keys().for_each(|op_code| {
            let name = self.0[op_code].name.clone();
            table.insert(name, *op_code);
        });
        return table;
    }

    // Return an instruction by its op code
    pub fn get(self, op: u8) -> Option<Instruction> {
        match self.0.get(&op) {
            Some(instr) => Some(instr.clone()),
            None => None,
        }
    }

    // Return an instruction by its name
    pub fn instr_by_name(self, name: &str) -> Option<Instruction> {
        match self.0.values().find(|instr| instr.name == name.to_string()) {
            Some(instr) => Some(instr.clone()),
            None => None,
        }
    }

    pub fn default() -> Self {
        let mut table = Self::new();
        table.push(Instruction::new(0, "halt", 0, halt));
        table.push(Instruction::new(1, "push", 1, push));
        table.push(Instruction::new(2, "pop", 0, pop));
        table.push(Instruction::new(3, "load", 1, load));
        table.push(Instruction::new(4, "store", 1, store));
        table.push(Instruction::new(5, "add", 0, add));
        table.push(Instruction::new(6, "sub", 0, subtract));
        table.push(Instruction::new(7, "mul", 0, multiply));
        table.push(Instruction::new(8, "div", 0, divide));
        table.push(Instruction::new(9, "mod", 0, modulo));
        table.push(Instruction::new(10, "eq", 0, equal));
        table.push(Instruction::new(11, "neq", 0, n_equal));
        table.push(Instruction::new(12, "lt", 0, l_than));
        table.push(Instruction::new(13, "lte", 0, l_than_eq));
        table.push(Instruction::new(14, "gt", 0, g_than));
        table.push(Instruction::new(15, "gte", 0, g_than_eq));
        return table;
    }
}
