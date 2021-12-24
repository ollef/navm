mod add;
pub use crate::block::add::*;
mod and_then;
pub use crate::block::and_then::*;
use crate::graph::*;

use std::collections::HashSet;
use std::hash::Hash;

#[derive(Clone)]
pub struct BlockOO<Instruction> {
    pub instructions: Vec<Instruction>,
}

#[derive(Clone)]
pub struct BlockOC<Instruction, Terminator> {
    pub instructions: Vec<Instruction>,
    pub terminator: Terminator,
}

#[derive(Clone)]
pub struct BlockCO<Initiator, Instruction> {
    pub initiator: Initiator,
    pub instructions: Vec<Instruction>,
}

#[derive(Clone)]
pub struct BlockCC<Initiator, Instruction, Terminator> {
    pub initiator: Initiator,
    pub instructions: Vec<Instruction>,
    pub terminator: Terminator,
}

pub trait Terminate<Label> {
    fn successors(self: &Self) -> HashSet<Label>;
}

impl<Instruction> BlockOO<Instruction> {
    pub fn new() -> Self {
        BlockOO {
            instructions: Vec::new(),
        }
    }
}

impl<Instruction> From<Instruction> for BlockOO<Instruction> {
    fn from(i: Instruction) -> Self {
        BlockOO {
            instructions: vec![i],
        }
    }
}

impl<Instruction, Terminator> From<Terminator> for BlockOC<Instruction, Terminator> {
    fn from(t: Terminator) -> Self {
        BlockOC {
            instructions: vec![],
            terminator: t,
        }
    }
}

impl<Initiator, Instruction> From<Initiator> for BlockCO<Initiator, Instruction> {
    fn from(i: Initiator) -> Self {
        BlockCO {
            initiator: i,
            instructions: vec![],
        }
    }
}

impl<Label, Instruction, Terminator> Terminate<Label> for BlockOC<Instruction, Terminator>
where
    Terminator: Terminate<Label>,
{
    fn successors(&self) -> HashSet<Label> {
        self.terminator.successors()
    }
}

impl<Label, Initiator, Instruction, Terminator> Terminate<Label>
    for BlockCC<Initiator, Instruction, Terminator>
where
    Terminator: Terminate<Label>,
{
    fn successors(&self) -> HashSet<Label> {
        self.terminator.successors()
    }
}
