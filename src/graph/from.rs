use crate::graph::*;

impl<Label, Initiator, Instruction, Terminator> From<Instruction>
    for GraphOO<Label, Initiator, Instruction, Terminator>
{
    fn from(i: Instruction) -> GraphOO<Label, Initiator, Instruction, Terminator> {
        GraphOO::Single(BlockOO::from(i))
    }
}

impl<Label, Initiator, Instruction, Terminator> From<Terminator>
    for GraphOC<Label, Initiator, Instruction, Terminator>
{
    fn from(t: Terminator) -> GraphOC<Label, Initiator, Instruction, Terminator> {
        GraphOC {
            entry: BlockOC::from(t),
            labels: Labels::new(),
        }
    }
}

impl<Label, Initiator, Instruction, Terminator> From<(Label, Initiator)>
    for GraphCO<Label, Initiator, Instruction, Terminator>
{
    fn from(
        (label, initiator): (Label, Initiator),
    ) -> GraphCO<Label, Initiator, Instruction, Terminator> {
        GraphCO {
            labels: Labels::new(),
            exit_label: label,
            exit: BlockCO::from(initiator),
        }
    }
}
