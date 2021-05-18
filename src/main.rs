use std::array::IntoIter;
use std::collections::HashSet;
use std::iter::FromIterator;

mod cfg;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Label {
    unique: i32,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Local {
    unique: i32,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Constant {
    I32(i32),
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Operand {
    Local(Local),
    Constant(Constant),
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Instruction {
    Add(Local, Operand, Operand),
    Mul(Local, Operand, Operand),
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Terminator {
    Jump(Label),
    Conditional(Operand, Label, Label),
}

impl cfg::Terminate<Label> for Terminator {
    fn successors(&self) -> std::collections::HashSet<Label> {
        match self {
            Terminator::Jump(label) => HashSet::from_iter(IntoIter::new([*label])),
            Terminator::Conditional(_, label1, label2) => {
                HashSet::from_iter(IntoIter::new([*label1, *label2]))
            }
        }
    }
}

fn main() {
    println!("Hello, world!");
}
