mod add;
pub use crate::graph::add::*;
mod analysis;
pub use crate::graph::analysis::*;
mod and_then;
pub use crate::graph::and_then::*;
mod postorder;
pub use crate::graph::postorder::*;
mod with_replacement;

pub use crate::block::*;

use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::hash::Hash;
use std::ops::Add;

#[derive(Clone)]
pub struct Labels<Label, Initiator, Instruction, Terminator> {
    pub map: HashMap<Label, BlockCC<Initiator, Instruction, Terminator>>,
}

#[derive(Clone)]
pub enum GraphOO<Label, Initiator, Instruction, Terminator> {
    Single(BlockOO<Instruction>),
    Many {
        entry: BlockOC<Instruction, Terminator>,
        labels: Labels<Label, Initiator, Instruction, Terminator>,
        exit_label: Label,
        exit: BlockCO<Initiator, Instruction>,
    },
}

#[derive(Clone)]
pub struct GraphOC<Label, Initiator, Instruction, Terminator> {
    pub entry: BlockOC<Instruction, Terminator>,
    pub labels: Labels<Label, Initiator, Instruction, Terminator>,
}

#[derive(Clone)]
pub struct GraphCO<Label, Initiator, Instruction, Terminator> {
    pub labels: Labels<Label, Initiator, Instruction, Terminator>,
    pub exit_label: Label,
    pub exit: BlockCO<Initiator, Instruction>,
}

#[derive(Clone)]
pub struct GraphCC<Label, Initiator, Instruction, Terminator> {
    pub labels: Labels<Label, Initiator, Instruction, Terminator>,
}

impl<Label, Initiator, Instruction, Terminator> Labels<Label, Initiator, Instruction, Terminator> {
    pub fn new() -> Self {
        Labels {
            map: HashMap::new(),
        }
    }
}

impl<Label, Initiator, Instruction, Terminator> GraphOO<Label, Initiator, Instruction, Terminator> {
    pub fn new() -> Self {
        GraphOO::Single(BlockOO::new())
    }
}

impl<Label, Initiator, Instruction, Terminator> From<Instruction>
    for GraphOO<Label, Initiator, Instruction, Terminator>
{
    fn from(i: Instruction) -> Self {
        GraphOO::Single(BlockOO::from(i))
    }
}

impl<Label, Initiator, Instruction, Terminator> From<Terminator>
    for GraphOC<Label, Initiator, Instruction, Terminator>
{
    fn from(t: Terminator) -> Self {
        GraphOC {
            entry: BlockOC::from(t),
            labels: Labels::new(),
        }
    }
}

impl<Label, Initiator, Instruction, Terminator> From<(Label, Initiator)>
    for GraphCO<Label, Initiator, Instruction, Terminator>
{
    fn from((label, initiator): (Label, Initiator)) -> Self {
        GraphCO {
            labels: Labels::new(),
            exit_label: label,
            exit: BlockCO::from(initiator),
        }
    }
}
