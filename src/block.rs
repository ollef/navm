use crate::graph::*;
use std::hash::Hash;
use std::ops::Add;

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

impl<Instruction> Add<Instruction> for BlockOO<Instruction> {
    type Output = BlockOO<Instruction>;
    fn add(mut self, instruction: Instruction) -> Self::Output {
        self.instructions.push(instruction);
        self
    }
}

impl<Initiator, Instruction> Add<Instruction> for BlockCO<Initiator, Instruction> {
    type Output = BlockCO<Initiator, Instruction>;
    fn add(mut self, instruction: Instruction) -> Self::Output {
        self.instructions.push(instruction);
        self
    }
}

impl<Instruction> Add<BlockOO<Instruction>> for BlockOO<Instruction> {
    type Output = BlockOO<Instruction>;
    fn add(mut self, mut rhs: BlockOO<Instruction>) -> Self::Output {
        self.instructions.append(&mut rhs.instructions);
        self
    }
}

impl<Instruction, Terminator> Add<BlockOC<Instruction, Terminator>> for BlockOO<Instruction> {
    type Output = BlockOC<Instruction, Terminator>;
    fn add(mut self, mut rhs: BlockOC<Instruction, Terminator>) -> Self::Output {
        self.instructions.append(&mut rhs.instructions);
        BlockOC {
            instructions: self.instructions,
            terminator: rhs.terminator,
        }
    }
}

impl<Initiator, Instruction, Terminator> Add<BlockOC<Instruction, Terminator>>
    for BlockCO<Initiator, Instruction>
{
    type Output = BlockCC<Initiator, Instruction, Terminator>;
    fn add(mut self, mut rhs: BlockOC<Instruction, Terminator>) -> Self::Output {
        self.instructions.append(&mut rhs.instructions);
        BlockCC {
            initiator: self.initiator,
            instructions: self.instructions,
            terminator: rhs.terminator,
        }
    }
}

impl<Initiator, Instruction> Add<BlockOO<Instruction>> for BlockCO<Initiator, Instruction> {
    type Output = BlockCO<Initiator, Instruction>;
    fn add(mut self, mut rhs: BlockOO<Instruction>) -> Self::Output {
        self.instructions.append(&mut rhs.instructions);
        self
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
        Label2: Eq + Hash + Clone,
        BindInstruction: Fn(&Instruction) -> GraphOO<Label2, Initiator2, Instruction2, Terminator2>,
    {
        self.instructions
            .iter()
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
        Label2: Eq + Hash + Clone,
        BindInstruction: Fn(&Instruction) -> GraphOO<Label2, Initiator2, Instruction2, Terminator2>,
        BindTerminator: Fn(&Terminator) -> GraphOC<Label2, Initiator2, Instruction2, Terminator2>,
    {
        self.instructions
            .iter()
            .fold(GraphOO::new(), |graph, i| graph + bind_instruction(i))
            + bind_terminator(&self.terminator)
    }
}

impl<Initiator, Instruction> BlockCO<Initiator, Instruction> {
    pub fn and_then<
        Label,
        Label2,
        Initiator2,
        Instruction2,
        Terminator2,
        BindInitiator,
        BindInstruction,
    >(
        &self,
        label: &Label,
        bind_initiator: BindInitiator,
        bind_instruction: BindInstruction,
    ) -> GraphCO<Label2, Initiator2, Instruction2, Terminator2>
    where
        Label2: Eq + Hash + Clone,
        BindInitiator:
            Fn(&Label, &Initiator) -> GraphCO<Label2, Initiator2, Instruction2, Terminator2>,
        BindInstruction: Fn(&Instruction) -> GraphOO<Label2, Initiator2, Instruction2, Terminator2>,
    {
        bind_initiator(label, &self.initiator)
            + self
                .instructions
                .iter()
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
        Label2: Eq + Hash + Clone,
        BindInitiator: Fn(&Initiator) -> GraphCO<Label2, Initiator2, Instruction2, Terminator2>,
        BindInstruction: Fn(&Instruction) -> GraphOO<Label2, Initiator2, Instruction2, Terminator2>,
        BindTerminator: Fn(&Terminator) -> GraphOC<Label2, Initiator2, Instruction2, Terminator2>,
    {
        bind_initiator(&self.initiator)
            + self
                .instructions
                .iter()
                .fold(GraphOO::new(), |graph, i| graph + bind_instruction(i))
            + bind_terminator(&self.terminator)
    }
}
