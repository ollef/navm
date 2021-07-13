mod add;
pub use crate::block::add::*;
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

impl<Instruction> From<Instruction> for BlockOO<Instruction> {
    fn from(i: Instruction) -> BlockOO<Instruction> {
        BlockOO {
            instructions: vec![i],
        }
    }
}

impl<Instruction, Terminator> From<Terminator> for BlockOC<Instruction, Terminator> {
    fn from(t: Terminator) -> BlockOC<Instruction, Terminator> {
        BlockOC {
            instructions: vec![],
            terminator: t,
        }
    }
}

impl<Initiator, Instruction> From<Initiator> for BlockCO<Initiator, Instruction> {
    fn from(i: Initiator) -> BlockCO<Initiator, Instruction> {
        BlockCO {
            initiator: i,
            instructions: vec![],
        }
    }
}

impl<Instruction> BlockOO<Instruction> {
    pub fn new() -> BlockOO<Instruction> {
        BlockOO {
            instructions: Vec::new(),
        }
    }

    pub fn and_then<Label2, Initiator2, Instruction2, Terminator2, BindInstruction>(
        &self,
        bind_instruction: BindInstruction,
    ) -> GraphOO<Label2, Initiator2, Instruction2, Terminator2>
    where
        Label2: Eq + Hash,
        BindInstruction: Fn(&Instruction) -> GraphOO<Label2, Initiator2, Instruction2, Terminator2>,
    {
        self.instructions
            .iter()
            .fold(GraphOO::new(), |graph, i| graph + bind_instruction(i))
    }

    pub fn and_then_into<Label2, Initiator2, Instruction2, Terminator2, BindInstruction>(
        self,
        bind_instruction: BindInstruction,
    ) -> GraphOO<Label2, Initiator2, Instruction2, Terminator2>
    where
        Label2: Eq + Hash,
        BindInstruction: Fn(Instruction) -> GraphOO<Label2, Initiator2, Instruction2, Terminator2>,
    {
        self.instructions
            .into_iter()
            .fold(GraphOO::new(), |graph, i| graph + bind_instruction(i))
    }
}

impl<Instruction, Terminator> BlockOC<Instruction, Terminator> {
    pub fn and_then<
        Label2,
        Initiator2,
        Instruction2,
        Terminator2,
        BindInstruction,
        BindTerminator,
    >(
        &self,
        bind_instruction: BindInstruction,
        bind_terminator: BindTerminator,
    ) -> GraphOC<Label2, Initiator2, Instruction2, Terminator2>
    where
        Label2: Eq + Hash,
        BindInstruction: Fn(&Instruction) -> GraphOO<Label2, Initiator2, Instruction2, Terminator2>,
        BindTerminator: Fn(&Terminator) -> GraphOC<Label2, Initiator2, Instruction2, Terminator2>,
    {
        self.instructions
            .iter()
            .fold(GraphOO::new(), |graph, i| graph + bind_instruction(i))
            + bind_terminator(&self.terminator)
    }

    pub fn and_then_into<
        Label2,
        Initiator2,
        Instruction2,
        Terminator2,
        BindInstruction,
        BindTerminator,
    >(
        self,
        bind_instruction: BindInstruction,
        bind_terminator: BindTerminator,
    ) -> GraphOC<Label2, Initiator2, Instruction2, Terminator2>
    where
        Label2: Eq + Hash,
        BindInstruction: Fn(Instruction) -> GraphOO<Label2, Initiator2, Instruction2, Terminator2>,
        BindTerminator: Fn(Terminator) -> GraphOC<Label2, Initiator2, Instruction2, Terminator2>,
    {
        self.instructions
            .into_iter()
            .fold(GraphOO::new(), |graph, i| graph + bind_instruction(i))
            + bind_terminator(self.terminator)
    }
}

impl<Initiator, Instruction> BlockCO<Initiator, Instruction> {
    pub fn and_then<Label2, Initiator2, Instruction2, Terminator2, BindInitiator, BindInstruction>(
        &self,
        bind_initiator: BindInitiator,
        bind_instruction: BindInstruction,
    ) -> GraphCO<Label2, Initiator2, Instruction2, Terminator2>
    where
        Label2: Eq + Hash,
        BindInitiator: FnOnce(&Initiator) -> GraphCO<Label2, Initiator2, Instruction2, Terminator2>,
        BindInstruction: Fn(&Instruction) -> GraphOO<Label2, Initiator2, Instruction2, Terminator2>,
    {
        bind_initiator(&self.initiator)
            + self
                .instructions
                .iter()
                .fold(GraphOO::new(), |graph, i| graph + bind_instruction(i))
    }

    pub fn and_then_into<
        Label2,
        Initiator2,
        Instruction2,
        Terminator2,
        BindInitiator,
        BindInstruction,
    >(
        self,
        bind_initiator: BindInitiator,
        bind_instruction: BindInstruction,
    ) -> GraphCO<Label2, Initiator2, Instruction2, Terminator2>
    where
        Label2: Eq + Hash,
        BindInitiator: FnOnce(Initiator) -> GraphCO<Label2, Initiator2, Instruction2, Terminator2>,
        BindInstruction: Fn(Instruction) -> GraphOO<Label2, Initiator2, Instruction2, Terminator2>,
    {
        bind_initiator(self.initiator)
            + self
                .instructions
                .into_iter()
                .fold(GraphOO::new(), |graph, i| graph + bind_instruction(i))
    }
}

impl<Initiator, Instruction, Terminator> BlockCC<Initiator, Instruction, Terminator> {
    pub fn and_then<
        Label2,
        Initiator2,
        Instruction2,
        Terminator2,
        BindInitiator,
        BindInstruction,
        BindTerminator,
    >(
        &self,
        bind_initiator: BindInitiator,
        bind_instruction: BindInstruction,
        bind_terminator: BindTerminator,
    ) -> GraphCC<Label2, Initiator2, Instruction2, Terminator2>
    where
        Label2: Eq + Hash,
        BindInitiator: FnOnce(&Initiator) -> GraphCO<Label2, Initiator2, Instruction2, Terminator2>,
        BindInstruction: Fn(&Instruction) -> GraphOO<Label2, Initiator2, Instruction2, Terminator2>,
        BindTerminator:
            FnOnce(&Terminator) -> GraphOC<Label2, Initiator2, Instruction2, Terminator2>,
    {
        bind_initiator(&self.initiator)
            + self
                .instructions
                .iter()
                .fold(GraphOO::new(), |graph, i| graph + bind_instruction(i))
            + bind_terminator(&self.terminator)
    }

    pub fn and_then_into<
        Label2,
        Initiator2,
        Instruction2,
        Terminator2,
        BindInitiator,
        BindInstruction,
        BindTerminator,
    >(
        self,
        bind_initiator: BindInitiator,
        bind_instruction: BindInstruction,
        bind_terminator: BindTerminator,
    ) -> GraphCC<Label2, Initiator2, Instruction2, Terminator2>
    where
        Label2: Eq + Hash,
        BindInitiator: FnOnce(Initiator) -> GraphCO<Label2, Initiator2, Instruction2, Terminator2>,
        BindInstruction: Fn(Instruction) -> GraphOO<Label2, Initiator2, Instruction2, Terminator2>,
        BindTerminator:
            FnOnce(Terminator) -> GraphOC<Label2, Initiator2, Instruction2, Terminator2>,
    {
        bind_initiator(self.initiator)
            + self
                .instructions
                .into_iter()
                .fold(GraphOO::new(), |graph, i| graph + bind_instruction(i))
            + bind_terminator(self.terminator)
    }
}
