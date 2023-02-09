use crate::{machine::Machine, constant::Constant};

// halts execution
pub fn halt(machine: &mut Machine, args: &[Constant]) {
    machine.halt();
}

// loads a value from the heap into the stack
pub fn load(machine: &mut Machine, args: &[Constant]) {
    let idx: usize = args[0].into();
    let value = machine.load(idx);
    machine.push(value);
}

// stores a value from the stack into the heap
pub fn store(machine: &mut Machine, args: &[Constant]) {
    let value = machine.pop();
    let idx: usize = args[0].into();
    machine.store(value, idx);
}

// pushes a value onto the stack
pub fn push(machine: &mut Machine, args: &[Constant]) {
    let value = args[0];
    machine.push(value);
}

// pops a value off the stack
pub fn pop(machine: &mut Machine, args: &[Constant]) {
    machine.pop();
}

// adds two values on the stack and pushes the result
pub fn add(machine: &mut Machine, args: &[Constant]) {
    let lhs = machine.pop();
    let rhs = machine.pop();
    match lhs + rhs {
        Ok(value) => machine.push(value),
        Err(err) => panic!("Error: {}", err),
    };
}

// subtracts two values on the stack and pushes the result
pub fn subtract(machine: &mut Machine, args: &[Constant]) {
    let lhs = machine.pop();
    let rhs = machine.pop();
    match lhs - rhs {
        Ok(value) => machine.push(value),
        Err(err) => panic!("Error: {}", err),
    };
}

// multiplies two values on the stack and pushes the result
pub fn multiply(machine: &mut Machine, args: &[Constant]) {
    let lhs = machine.pop();
    let rhs = machine.pop();
    match lhs * rhs {
        Ok(value) => machine.push(value),
        Err(err) => panic!("Error: {}", err),
    };
}

// divides two values on the stack and pushes the result
pub fn divide(machine: &mut Machine, args: &[Constant]) {
    let lhs = machine.pop();
    let rhs = machine.pop();
    match lhs / rhs {
        Ok(value) => machine.push(value),
        Err(err) => panic!("Error: {}", err),
    };
}

// divides two values on the stack and pushes the remainder
pub fn modulo(machine: &mut Machine, args: &[Constant]) {
    let lhs = machine.pop();
    let rhs = machine.pop();
    match lhs % rhs {
        Ok(value) => machine.push(value),
        Err(err) => panic!("Error: {}", err),
    };
}

// checks whether two values on the stack are equal and pushes the result
pub fn equal(machine: &mut Machine, args: &[Constant]) {
    let lhs = machine.pop();
    let rhs = machine.pop();
    machine.push((lhs == rhs).into());
}

// checks whether two values on the stack are not equal and pushes the result
pub fn n_equal(machine: &mut Machine, args: &[Constant]) {
    let lhs = machine.pop();
    let rhs = machine.pop();
    machine.push((lhs != rhs).into());
}

// checks whether lhs is less than rhs and pushes the result
pub fn l_than(machine: &mut Machine, args: &[Constant]) {
    let lhs = machine.pop();
    let rhs = machine.pop();
    machine.push((lhs < rhs).into());
}

// checks whether lhs is less than or equal to rhs and pushes the result
pub fn l_than_eq(machine: &mut Machine, args: &[Constant]) {
    let lhs = machine.pop();
    let rhs = machine.pop();
    machine.push((lhs <= rhs).into());
}

// checks whether lhs is greater than rhs and pushes the result
pub fn g_than(machine: &mut Machine, args: &[Constant]) {
    let lhs = machine.pop();
    let rhs = machine.pop();
    machine.push((lhs > rhs).into());
}

// checks whether lhs is greater than or equal to rhs and pushes the result
pub fn g_than_eq(machine: &mut Machine, args: &[Constant]) {
    let lhs = machine.pop();
    let rhs = machine.pop();
    machine.push((lhs >= rhs).into());
}

