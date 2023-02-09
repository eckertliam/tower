use std::mem::transmute;

use constant::Constant;

mod constant;
mod instruction;
mod instruction_table;
mod ops;
mod machine;

fn main() {
    println!("{}", Constant::from(1) == Constant::from(1));
}