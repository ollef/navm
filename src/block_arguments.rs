use crate::block;
use crate::graph;

pub type Initiator<Register> = Vec<Register>;

pub type BlockOO<Instruction> = block::BlockOO<Instruction>;
pub type BlockOC<Instruction, Terminator> = block::BlockOC<Instruction, Terminator>;
pub type BlockCO<Register, Instruction> = block::BlockCO<Initiator<Register>, Instruction>;
pub type BlockCC<Register, Instruction, Terminator> =
    block::BlockCC<Initiator<Register>, Instruction, Terminator>;

pub type GraphOO<Label, Register, Instruction, Terminator> =
    graph::GraphOO<Label, Initiator<Register>, Instruction, Terminator>;
pub type GraphOC<Label, Register, Instruction, Terminator> =
    graph::GraphOC<Label, Initiator<Register>, Instruction, Terminator>;
pub type GraphCO<Label, Register, Instruction, Terminator> =
    graph::GraphCO<Label, Initiator<Register>, Instruction, Terminator>;
pub type GraphCC<Label, Register, Instruction, Terminator> =
    graph::GraphCC<Label, Initiator<Register>, Instruction, Terminator>;
