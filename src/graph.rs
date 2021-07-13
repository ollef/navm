mod add;
pub use crate::graph::add::*;
mod analysis;
pub use crate::graph::analysis::*;
mod and_then;
pub use crate::graph::and_then::*;
mod from;
pub use crate::graph::from::*;
mod new;
pub use crate::graph::new::*;
mod postorder;
pub use crate::graph::postorder::*;

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

struct InitiatorWithReplacement<Label, Initiator, Instruction, Terminator> {
    original: Initiator,
    replacement: Option<Box<GraphCOWithReplacement<Label, Initiator, Instruction, Terminator>>>,
}

struct InstructionWithReplacement<Label, Initiator, Instruction, Terminator> {
    original: Instruction,
    replacement: Option<Box<GraphOOWithReplacement<Label, Initiator, Instruction, Terminator>>>,
}

struct TerminatorWithReplacement<Label, Initiator, Instruction, Terminator> {
    original: Terminator,
    replacement: Option<Box<GraphOCWithReplacement<Label, Initiator, Instruction, Terminator>>>,
}

type GraphOOWithReplacement<Label, Initiator, Instruction, Terminator> = GraphOO<
    Label,
    InitiatorWithReplacement<Label, Initiator, Instruction, Terminator>,
    InstructionWithReplacement<Label, Initiator, Instruction, Terminator>,
    TerminatorWithReplacement<Label, Initiator, Instruction, Terminator>,
>;

type GraphOCWithReplacement<Label, Initiator, Instruction, Terminator> = GraphOC<
    Label,
    InitiatorWithReplacement<Label, Initiator, Instruction, Terminator>,
    InstructionWithReplacement<Label, Initiator, Instruction, Terminator>,
    TerminatorWithReplacement<Label, Initiator, Instruction, Terminator>,
>;

type GraphCOWithReplacement<Label, Initiator, Instruction, Terminator> = GraphCO<
    Label,
    InitiatorWithReplacement<Label, Initiator, Instruction, Terminator>,
    InstructionWithReplacement<Label, Initiator, Instruction, Terminator>,
    TerminatorWithReplacement<Label, Initiator, Instruction, Terminator>,
>;

type GraphCCWithReplacement<Label, Initiator, Instruction, Terminator> = GraphCC<
    Label,
    InitiatorWithReplacement<Label, Initiator, Instruction, Terminator>,
    InstructionWithReplacement<Label, Initiator, Instruction, Terminator>,
    TerminatorWithReplacement<Label, Initiator, Instruction, Terminator>,
>;

impl<Label, Initiator, Instruction, Terminator>
    GraphOO<
        Label,
        InitiatorWithReplacement<Label, Initiator, Instruction, Terminator>,
        InstructionWithReplacement<Label, Initiator, Instruction, Terminator>,
        TerminatorWithReplacement<Label, Initiator, Instruction, Terminator>,
    >
where
    Label: Eq + Hash,
{
    fn replace(self) -> GraphOO<Label, Initiator, Instruction, Terminator> {
        self.and_then_into(
            &|label, initiator| match initiator.replacement {
                None => GraphCO::from((label, initiator.original)),
                Some(replacement) => (*replacement).replace(),
            },
            &|instruction| match instruction.replacement {
                None => GraphOO::from(instruction.original),
                Some(replacement) => (*replacement).replace(),
            },
            &|terminator| match terminator.replacement {
                None => GraphOC::from(terminator.original),
                Some(replacement) => (*replacement).replace(),
            },
        )
    }
}

impl<Label, Initiator, Instruction, Terminator>
    GraphOC<
        Label,
        InitiatorWithReplacement<Label, Initiator, Instruction, Terminator>,
        InstructionWithReplacement<Label, Initiator, Instruction, Terminator>,
        TerminatorWithReplacement<Label, Initiator, Instruction, Terminator>,
    >
where
    Label: Eq + Hash,
{
    fn replace(self) -> GraphOC<Label, Initiator, Instruction, Terminator> {
        self.and_then_into(
            &|label, initiator| match initiator.replacement {
                None => GraphCO::from((label, initiator.original)),
                Some(replacement) => (*replacement).replace(),
            },
            &|instruction| match instruction.replacement {
                None => GraphOO::from(instruction.original),
                Some(replacement) => (*replacement).replace(),
            },
            &|terminator| match terminator.replacement {
                None => GraphOC::from(terminator.original),
                Some(replacement) => (*replacement).replace(),
            },
        )
    }
}

impl<Label, Initiator, Instruction, Terminator>
    GraphCO<
        Label,
        InitiatorWithReplacement<Label, Initiator, Instruction, Terminator>,
        InstructionWithReplacement<Label, Initiator, Instruction, Terminator>,
        TerminatorWithReplacement<Label, Initiator, Instruction, Terminator>,
    >
where
    Label: Eq + Hash,
{
    fn replace(self) -> GraphCO<Label, Initiator, Instruction, Terminator> {
        self.and_then_into(
            &|label, initiator| match initiator.replacement {
                None => GraphCO::from((label, initiator.original)),
                Some(replacement) => (*replacement).replace(),
            },
            &|instruction| match instruction.replacement {
                None => GraphOO::from(instruction.original),
                Some(replacement) => (*replacement).replace(),
            },
            &|terminator| match terminator.replacement {
                None => GraphOC::from(terminator.original),
                Some(replacement) => (*replacement).replace(),
            },
        )
    }
}

impl<Label, Initiator, Instruction, Terminator>
    GraphCC<
        Label,
        InitiatorWithReplacement<Label, Initiator, Instruction, Terminator>,
        InstructionWithReplacement<Label, Initiator, Instruction, Terminator>,
        TerminatorWithReplacement<Label, Initiator, Instruction, Terminator>,
    >
where
    Initiator: Clone,
    Instruction: Clone,
    Terminator: Clone,
    Label: Eq + Hash + Clone,
{
    fn replace(self) -> GraphCC<Label, Initiator, Instruction, Terminator> {
        self.and_then_into(
            &|label, initiator| match initiator.replacement {
                None => GraphCO::from((label, initiator.original)),
                Some(replacement) => (*replacement).replace(),
            },
            &|instruction| match instruction.replacement {
                None => GraphOO::from(instruction.original),
                Some(replacement) => (*replacement).replace(),
            },
            &|terminator| match terminator.replacement {
                None => GraphOC::from(terminator.original),
                Some(replacement) => (*replacement).replace(),
            },
        )
    }
}
