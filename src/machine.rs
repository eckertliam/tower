use crate::{constant::Constant, instruction_table::InstructionTable, instruction::Instruction};

pub struct Machine {
    stack: Vec<Constant>,
    heap: Vec<Constant>,
    execution: bool,
    instr_table: InstructionTable,
    instr_ptr: usize,
    instr_stack: Vec<Instruction>,
}

impl Machine {
    pub fn new() -> Self {
        Self {
            stack: Vec::new(),
            heap: Vec::new(),
            execution: true,
            instr_table: InstructionTable::default(),
            instr_ptr: 0,
            instr_stack: Vec::new(),
        }
    }

    pub fn halt(&mut self) {
        self.execution = false;
    }

    pub fn push(&mut self, c: Constant) {
        self.stack.push(c)
    }

    pub fn pop(&mut self) -> Constant {
        self.stack.pop().expect("Error: unable to pop an empty stack")
    }

    pub fn load(&self, idx: usize) -> Constant {
        self.heap.get(idx).expect("Error: cannot load from empty address").clone()
    }

    pub fn store(&mut self, constant: Constant, idx: usize) {
        if self.heap.len() > idx {
            self.heap[idx] = constant;
        }else{
            self.heap.resize(idx - self.heap.len(), Constant::null());
        }
    }
}











