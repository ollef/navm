use crate::graph::*;

impl<Label, Initiator, Instruction, Terminator> Labels<Label, Initiator, Instruction, Terminator> {
    pub fn new() -> Labels<Label, Initiator, Instruction, Terminator> {
        Labels {
            map: HashMap::new(),
        }
    }
}

impl<Label, Initiator, Instruction, Terminator> GraphOO<Label, Initiator, Instruction, Terminator> {
    pub fn new() -> GraphOO<Label, Initiator, Instruction, Terminator> {
        GraphOO::Single(BlockOO::new())
    }
}
