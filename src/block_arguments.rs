use crate::graph;

pub type Initiator<Register> = Vec<Register>;

pub type BlockOO<Instruction> = graph::BlockOO<Instruction>;
pub type BlockOC<Instruction, Terminator> = graph::BlockOC<Instruction, Terminator>;
pub type BlockCO<Register, Instruction> = graph::BlockCO<Initiator<Register>, Instruction>;
pub type BlockCC<Register, Instruction, Terminator> =
    graph::BlockCC<Initiator<Register>, Instruction, Terminator>;

pub type GraphOO<Label, Register, Instruction, Terminator> =
    graph::GraphOO<Label, Initiator<Register>, Instruction, Terminator>;
pub type GraphOC<Label, Register, Instruction, Terminator> =
    graph::GraphOC<Label, Initiator<Register>, Instruction, Terminator>;
pub type GraphCO<Label, Register, Instruction, Terminator> =
    graph::GraphCO<Label, Initiator<Register>, Instruction, Terminator>;
pub type GraphCC<Label, Register, Instruction, Terminator> =
    graph::GraphCC<Label, Initiator<Register>, Instruction, Terminator>;
