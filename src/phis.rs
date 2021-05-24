use crate::graph;

pub struct Phi<Label, Register> {
    destination: Register,
    incomingValues: Vec<(Label, Register)>,
}
pub type Initiator<Label, Register> = Vec<Phi<Label, Register>>;

pub type BlockOO<Instruction> = graph::BlockOO<Instruction>;
pub type BlockOC<Instruction, Terminator> = graph::BlockOC<Instruction, Terminator>;
pub type BlockCO<Label, Register, Instruction> =
    graph::BlockCO<Initiator<Label, Register>, Instruction>;
pub type BlockCC<Label, Register, Instruction, Terminator> =
    graph::BlockCC<Initiator<Label, Register>, Instruction, Terminator>;

pub type GraphOO<Label, Register, Instruction, Terminator> =
    graph::GraphOO<Label, Initiator<Label, Register>, Instruction, Terminator>;
pub type GraphOC<Label, Register, Instruction, Terminator> =
    graph::GraphOC<Label, Initiator<Label, Register>, Instruction, Terminator>;
pub type GraphCO<Label, Register, Instruction, Terminator> =
    graph::GraphCO<Label, Initiator<Label, Register>, Instruction, Terminator>;
pub type GraphCC<Label, Register, Instruction, Terminator> =
    graph::GraphCC<Label, Initiator<Label, Register>, Instruction, Terminator>;
