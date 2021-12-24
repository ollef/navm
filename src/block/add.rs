use crate::block::*;
use std::ops::Add;

impl<Instruction> Add<Instruction> for BlockOO<Instruction> {
    type Output = Self;
    fn add(mut self, instruction: Instruction) -> Self::Output {
        self.instructions.push(instruction);
        self
    }
}

impl<Initiator, Instruction> Add<Instruction> for BlockCO<Initiator, Instruction> {
    type Output = Self;
    fn add(mut self, instruction: Instruction) -> Self::Output {
        self.instructions.push(instruction);
        self
    }
}

impl<Instruction> Add<BlockOO<Instruction>> for BlockOO<Instruction> {
    type Output = Self;
    fn add(mut self, mut rhs: Self) -> Self::Output {
        self.instructions.append(&mut rhs.instructions);
        self
    }
}

impl<Instruction, Terminator> Add<BlockOC<Instruction, Terminator>> for BlockOO<Instruction> {
    type Output = BlockOC<Instruction, Terminator>;
    fn add(mut self, mut rhs: Self::Output) -> Self::Output {
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
