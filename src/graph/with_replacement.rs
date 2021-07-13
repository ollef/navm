use crate::graph;
use std::hash::Hash;

struct Initiator<Label, InitiatorT, InstructionT, TerminatorT> {
    original: InitiatorT,
    replacement: Option<Box<GraphCO<Label, InitiatorT, InstructionT, TerminatorT>>>,
}

struct Instruction<Label, InitiatorT, InstructionT, TerminatorT> {
    original: InstructionT,
    replacement: Option<Box<GraphOO<Label, InitiatorT, InstructionT, TerminatorT>>>,
}

struct Terminator<Label, InitiatorT, InstructionT, TerminatorT> {
    original: TerminatorT,
    replacement: Option<Box<GraphOC<Label, InitiatorT, InstructionT, TerminatorT>>>,
}

type GraphOO<Label, InitiatorT, InstructionT, TerminatorT> = graph::GraphOO<
    Label,
    Initiator<Label, InitiatorT, InstructionT, TerminatorT>,
    Instruction<Label, InitiatorT, InstructionT, TerminatorT>,
    Terminator<Label, InitiatorT, InstructionT, TerminatorT>,
>;

type GraphOC<Label, InitiatorT, InstructionT, TerminatorT> = graph::GraphOC<
    Label,
    Initiator<Label, InitiatorT, InstructionT, TerminatorT>,
    Instruction<Label, InitiatorT, InstructionT, TerminatorT>,
    Terminator<Label, InitiatorT, InstructionT, TerminatorT>,
>;

type GraphCO<Label, InitiatorT, InstructionT, TerminatorT> = graph::GraphCO<
    Label,
    Initiator<Label, InitiatorT, InstructionT, TerminatorT>,
    Instruction<Label, InitiatorT, InstructionT, TerminatorT>,
    Terminator<Label, InitiatorT, InstructionT, TerminatorT>,
>;

type GraphCC<Label, InitiatorT, InstructionT, TerminatorT> = graph::GraphCC<
    Label,
    Initiator<Label, InitiatorT, InstructionT, TerminatorT>,
    Instruction<Label, InitiatorT, InstructionT, TerminatorT>,
    Terminator<Label, InitiatorT, InstructionT, TerminatorT>,
>;

impl<Label, InitiatorT, InstructionT, TerminatorT>
    GraphOO<Label, InitiatorT, InstructionT, TerminatorT>
where
    Label: Eq + Hash,
{
    fn replace(self) -> graph::GraphOO<Label, InitiatorT, InstructionT, TerminatorT> {
        self.and_then_into(
            &|label, initiator| match initiator.replacement {
                None => graph::GraphCO::from((label, initiator.original)),
                Some(replacement) => (*replacement).replace(),
            },
            &|instruction| match instruction.replacement {
                None => graph::GraphOO::from(instruction.original),
                Some(replacement) => (*replacement).replace(),
            },
            &|terminator| match terminator.replacement {
                None => graph::GraphOC::from(terminator.original),
                Some(replacement) => (*replacement).replace(),
            },
        )
    }
}

impl<Label, InitiatorT, InstructionT, TerminatorT>
    GraphOC<Label, InitiatorT, InstructionT, TerminatorT>
where
    Label: Eq + Hash,
{
    fn replace(self) -> graph::GraphOC<Label, InitiatorT, InstructionT, TerminatorT> {
        self.and_then_into(
            &|label, initiator| match initiator.replacement {
                None => graph::GraphCO::from((label, initiator.original)),
                Some(replacement) => (*replacement).replace(),
            },
            &|instruction| match instruction.replacement {
                None => graph::GraphOO::from(instruction.original),
                Some(replacement) => (*replacement).replace(),
            },
            &|terminator| match terminator.replacement {
                None => graph::GraphOC::from(terminator.original),
                Some(replacement) => (*replacement).replace(),
            },
        )
    }
}

impl<Label, InitiatorT, InstructionT, TerminatorT>
    GraphCO<Label, InitiatorT, InstructionT, TerminatorT>
where
    Label: Eq + Hash,
{
    fn replace(self) -> graph::GraphCO<Label, InitiatorT, InstructionT, TerminatorT> {
        self.and_then_into(
            &|label, initiator| match initiator.replacement {
                None => graph::GraphCO::from((label, initiator.original)),
                Some(replacement) => (*replacement).replace(),
            },
            &|instruction| match instruction.replacement {
                None => graph::GraphOO::from(instruction.original),
                Some(replacement) => (*replacement).replace(),
            },
            &|terminator| match terminator.replacement {
                None => graph::GraphOC::from(terminator.original),
                Some(replacement) => (*replacement).replace(),
            },
        )
    }
}

impl<Label, InitiatorT, InstructionT, TerminatorT>
    GraphCC<Label, InitiatorT, InstructionT, TerminatorT>
where
    Label: Eq + Hash,
{
    fn replace(self) -> graph::GraphCC<Label, InitiatorT, InstructionT, TerminatorT> {
        self.and_then_into(
            &|label, initiator| match initiator.replacement {
                None => graph::GraphCO::from((label, initiator.original)),
                Some(replacement) => (*replacement).replace(),
            },
            &|instruction| match instruction.replacement {
                None => graph::GraphOO::from(instruction.original),
                Some(replacement) => (*replacement).replace(),
            },
            &|terminator| match terminator.replacement {
                None => graph::GraphOC::from(terminator.original),
                Some(replacement) => (*replacement).replace(),
            },
        )
    }
}
