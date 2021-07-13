mod from;
pub use crate::graph::from::*;
mod new;
pub use crate::graph::new::*;

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

impl<Label, Initiator, Instruction, Terminator>
    Add<GraphOO<Label, Initiator, Instruction, Terminator>>
    for GraphOO<Label, Initiator, Instruction, Terminator>
where
    Label: Eq + Hash + Clone,
{
    type Output = GraphOO<Label, Initiator, Instruction, Terminator>;
    fn add(self, other: GraphOO<Label, Initiator, Instruction, Terminator>) -> Self::Output {
        match (self, other) {
            (GraphOO::Single(block), other) => block + other,
            (self_, GraphOO::Single(block)) => self_ + block,
            (
                GraphOO::Many {
                    entry: entry1,
                    labels: mut labels1,
                    exit_label: exit_label1,
                    exit: exit1,
                },
                GraphOO::Many {
                    entry: entry2,
                    labels: labels2,
                    exit_label: exit_label2,
                    exit: exit2,
                },
            ) => {
                labels1.map.extend(labels2.map);
                labels1.map.insert(exit_label1, exit1 + entry2);
                GraphOO::Many {
                    entry: entry1,
                    labels: labels1,
                    exit_label: exit_label2,
                    exit: exit2,
                }
            }
        }
    }
}

impl<Label, Initiator, Instruction, Terminator>
    Add<GraphOC<Label, Initiator, Instruction, Terminator>>
    for GraphOO<Label, Initiator, Instruction, Terminator>
where
    Label: Eq + Hash + Clone,
{
    type Output = GraphOC<Label, Initiator, Instruction, Terminator>;
    fn add(self, other: GraphOC<Label, Initiator, Instruction, Terminator>) -> Self::Output {
        match self {
            GraphOO::Single(block) => block + other,
            GraphOO::Many {
                entry,
                mut labels,
                exit_label,
                exit,
            } => {
                labels.map.extend(other.labels.map);
                labels.map.insert(exit_label, exit + other.entry);
                GraphOC { entry, labels }
            }
        }
    }
}

impl<Label, Initiator, Instruction, Terminator>
    Add<GraphOO<Label, Initiator, Instruction, Terminator>>
    for GraphCO<Label, Initiator, Instruction, Terminator>
where
    Label: Eq + Hash + Clone,
{
    type Output = GraphCO<Label, Initiator, Instruction, Terminator>;
    fn add(mut self, other: GraphOO<Label, Initiator, Instruction, Terminator>) -> Self::Output {
        match other {
            GraphOO::Single(block) => self + block,
            GraphOO::Many {
                entry,
                labels,
                exit_label,
                exit,
            } => {
                self.labels.map.extend(labels.map);
                self.labels.map.insert(self.exit_label, self.exit + entry);
                GraphCO {
                    labels: self.labels,
                    exit_label,
                    exit,
                }
            }
        }
    }
}

impl<Label, Initiator, Instruction, Terminator>
    Add<GraphOC<Label, Initiator, Instruction, Terminator>>
    for GraphCO<Label, Initiator, Instruction, Terminator>
where
    Label: Eq + Hash + Clone,
{
    type Output = GraphCC<Label, Initiator, Instruction, Terminator>;
    fn add(mut self, other: GraphOC<Label, Initiator, Instruction, Terminator>) -> Self::Output {
        self.labels.map.extend(other.labels.map);
        self.labels
            .map
            .insert(self.exit_label, self.exit + other.entry);
        GraphCC {
            labels: self.labels,
        }
    }
}

impl<Label, Initiator, Instruction, Terminator>
    Add<GraphCO<Label, Initiator, Instruction, Terminator>>
    for GraphCC<Label, Initiator, Instruction, Terminator>
where
    Label: Eq + Hash + Clone,
{
    type Output = GraphCO<Label, Initiator, Instruction, Terminator>;
    fn add(mut self, other: GraphCO<Label, Initiator, Instruction, Terminator>) -> Self::Output {
        self.labels.map.extend(other.labels.map);
        GraphCO {
            labels: self.labels,
            exit_label: other.exit_label,
            exit: other.exit,
        }
    }
}

impl<Label, Initiator, Instruction, Terminator> Add<BlockOO<Instruction>>
    for GraphCO<Label, Initiator, Instruction, Terminator>
{
    type Output = GraphCO<Label, Initiator, Instruction, Terminator>;
    fn add(self, other: BlockOO<Instruction>) -> Self::Output {
        GraphCO {
            labels: self.labels,
            exit_label: self.exit_label,
            exit: self.exit + other,
        }
    }
}

impl<Label, Initiator, Instruction, Terminator> Add<BlockOO<Instruction>>
    for GraphOO<Label, Initiator, Instruction, Terminator>
{
    type Output = GraphOO<Label, Initiator, Instruction, Terminator>;
    fn add(self, other: BlockOO<Instruction>) -> Self::Output {
        match self {
            GraphOO::Single(block) => GraphOO::Single(block + other),
            GraphOO::Many {
                entry,
                labels,
                exit_label,
                exit,
            } => GraphOO::Many {
                entry,
                labels,
                exit_label,
                exit: exit + other,
            },
        }
    }
}

impl<Label, Initiator, Instruction, Terminator>
    Add<GraphOO<Label, Initiator, Instruction, Terminator>> for BlockOO<Instruction>
{
    type Output = GraphOO<Label, Initiator, Instruction, Terminator>;
    fn add(self, other: GraphOO<Label, Initiator, Instruction, Terminator>) -> Self::Output {
        match other {
            GraphOO::Single(block) => GraphOO::Single(self + block),
            GraphOO::Many {
                entry,
                labels,
                exit_label,
                exit,
            } => GraphOO::Many {
                entry: self + entry,
                labels,
                exit_label,
                exit,
            },
        }
    }
}

impl<Label, Initiator, Instruction, Terminator>
    Add<GraphOC<Label, Initiator, Instruction, Terminator>> for BlockOO<Instruction>
{
    type Output = GraphOC<Label, Initiator, Instruction, Terminator>;
    fn add(self, other: GraphOC<Label, Initiator, Instruction, Terminator>) -> Self::Output {
        GraphOC {
            entry: self + other.entry,
            labels: other.labels,
        }
    }
}

impl<Label, Initiator, Instruction, Terminator> Add<BlockOC<Instruction, Terminator>>
    for GraphOO<Label, Initiator, Instruction, Terminator>
where
    Label: Eq + Hash + Clone,
{
    type Output = GraphOC<Label, Initiator, Instruction, Terminator>;
    fn add(self, other: BlockOC<Instruction, Terminator>) -> Self::Output {
        match self {
            GraphOO::Single(block) => GraphOC {
                entry: block + other,
                labels: Labels::new(),
            },
            GraphOO::Many {
                entry,
                mut labels,
                exit_label,
                exit,
            } => {
                labels.map.insert(exit_label, exit + other);
                GraphOC { entry, labels }
            }
        }
    }
}

impl<Label, Initiator, Instruction, Terminator> GraphOO<Label, Initiator, Instruction, Terminator> {
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
        bind_initiator: &BindInitiator,
        bind_instruction: &BindInstruction,
        bind_terminator: &BindTerminator,
    ) -> GraphOO<Label2, Initiator2, Instruction2, Terminator2>
    where
        Label2: Eq + Hash + Clone,
        BindInitiator:
            Fn(&Label, &Initiator) -> GraphCO<Label2, Initiator2, Instruction2, Terminator2>,
        BindInstruction: Fn(&Instruction) -> GraphOO<Label2, Initiator2, Instruction2, Terminator2>,
        BindTerminator: Fn(&Terminator) -> GraphOC<Label2, Initiator2, Instruction2, Terminator2>,
    {
        match self {
            GraphOO::Single(block) => block.and_then(bind_instruction),
            GraphOO::Many {
                entry,
                labels,
                exit_label,
                exit,
            } => {
                let entry_graph = entry.and_then(bind_instruction, bind_terminator);
                let entry = entry_graph.entry;
                let mut labels = labels.and_then(bind_initiator, bind_instruction, bind_terminator);
                let exit_graph = exit.and_then(exit_label, bind_initiator, bind_instruction);
                labels.map.extend(entry_graph.labels.map);
                labels.map.extend(exit_graph.labels.map);
                GraphOO::Many {
                    entry,
                    labels,
                    exit_label: exit_graph.exit_label,
                    exit: exit_graph.exit,
                }
            }
        }
    }
}

impl<Label, Initiator, Instruction, Terminator> GraphCO<Label, Initiator, Instruction, Terminator> {
    fn and_then<
        Label2,
        Initiator2,
        Instruction2,
        Terminator2,
        BindInitiator,
        BindInstruction,
        BindTerminator,
    >(
        &self,
        bind_initiator: &BindInitiator,
        bind_instruction: &BindInstruction,
        bind_terminator: &BindTerminator,
    ) -> GraphCO<Label2, Initiator2, Instruction2, Terminator2>
    where
        Label2: Eq + Hash + Clone,
        BindInitiator:
            Fn(&Label, &Initiator) -> GraphCO<Label2, Initiator2, Instruction2, Terminator2>,
        BindInstruction: Fn(&Instruction) -> GraphOO<Label2, Initiator2, Instruction2, Terminator2>,
        BindTerminator: Fn(&Terminator) -> GraphOC<Label2, Initiator2, Instruction2, Terminator2>,
    {
        let labels = self
            .labels
            .and_then(bind_initiator, bind_instruction, bind_terminator);
        let exit_graph = self
            .exit
            .and_then(&self.exit_label, bind_initiator, bind_instruction);
        GraphCC { labels } + exit_graph
    }
}

impl<Label, Initiator, Instruction, Terminator> GraphOC<Label, Initiator, Instruction, Terminator> {
    fn and_then<
        Label2,
        Initiator2,
        Instruction2,
        Terminator2,
        BindInitiator,
        BindInstruction,
        BindTerminator,
    >(
        &self,
        bind_initiator: &BindInitiator,
        bind_instruction: &BindInstruction,
        bind_terminator: &BindTerminator,
    ) -> GraphOC<Label2, Initiator2, Instruction2, Terminator2>
    where
        Label2: Eq + Hash + Clone,
        BindInitiator:
            Fn(&Label, &Initiator) -> GraphCO<Label2, Initiator2, Instruction2, Terminator2>,
        BindInstruction: Fn(&Instruction) -> GraphOO<Label2, Initiator2, Instruction2, Terminator2>,
        BindTerminator: Fn(&Terminator) -> GraphOC<Label2, Initiator2, Instruction2, Terminator2>,
    {
        let entry_graph = self.entry.and_then(bind_instruction, bind_terminator);
        let entry = entry_graph.entry;
        let mut labels = self
            .labels
            .and_then(bind_initiator, bind_instruction, bind_terminator);
        labels.map.extend(entry_graph.labels.map);
        GraphOC { entry, labels }
    }
}

impl<Label, Initiator, Instruction, Terminator> GraphCC<Label, Initiator, Instruction, Terminator> {
    fn and_then<
        Label2,
        Initiator2,
        Instruction2,
        Terminator2,
        BindInitiator,
        BindInstruction,
        BindTerminator,
    >(
        &self,
        bind_initiator: &BindInitiator,
        bind_instruction: &BindInstruction,
        bind_terminator: &BindTerminator,
    ) -> GraphCC<Label2, Initiator2, Instruction2, Terminator2>
    where
        Label2: Eq + Hash + Clone,
        BindInitiator:
            Fn(&Label, &Initiator) -> GraphCO<Label2, Initiator2, Instruction2, Terminator2>,
        BindInstruction: Fn(&Instruction) -> GraphOO<Label2, Initiator2, Instruction2, Terminator2>,
        BindTerminator: Fn(&Terminator) -> GraphOC<Label2, Initiator2, Instruction2, Terminator2>,
    {
        let mut labels = self
            .labels
            .and_then(bind_initiator, bind_instruction, bind_terminator);
        GraphCC { labels }
    }
}

impl<Label, Initiator, Instruction, Terminator> Labels<Label, Initiator, Instruction, Terminator> {
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
        bind_initiator: &BindInitiator,
        bind_instruction: &BindInstruction,
        bind_terminator: &BindTerminator,
    ) -> Labels<Label2, Initiator2, Instruction2, Terminator2>
    where
        Label2: Eq + Hash + Clone,
        BindInitiator:
            Fn(&Label, &Initiator) -> GraphCO<Label2, Initiator2, Instruction2, Terminator2>,
        BindInstruction: Fn(&Instruction) -> GraphOO<Label2, Initiator2, Instruction2, Terminator2>,
        BindTerminator: Fn(&Terminator) -> GraphOC<Label2, Initiator2, Instruction2, Terminator2>,
    {
        let mut result = Labels::new();
        for (label, block) in self.map.iter() {
            let graph = block.and_then(
                |initiator| bind_initiator(label, initiator),
                bind_instruction,
                bind_terminator,
            );
            result.map.extend(graph.labels.map);
        }
        result
    }
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

pub trait Fact {
    fn bottom() -> Self;
    fn join(self: &mut Self, fact: &Self);
}

struct Postorder<'a, Label, Initiator, Instruction, Terminator> {
    todo: Vec<Label>,
    visited: HashSet<Label>,
    graph: &'a GraphOC<Label, Initiator, Instruction, Terminator>,
}

impl<'a, Label, Initiator, Instruction, Terminator> Iterator
    for Postorder<'a, Label, Initiator, Instruction, Terminator>
where
    Terminator: Terminate<Label>,
    Label: Eq + Hash + Clone,
{
    type Item = (Label, &'a BlockCC<Initiator, Instruction, Terminator>);
    fn next(&mut self) -> Option<Self::Item> {
        let mut tail = true;
        while let Some(label) = self.todo.last().map(|x| x.clone()) {
            if let Some(block) = self.graph.labels.map.get(&label) {
                for successor in block.terminator.successors() {
                    if self.visited.insert(successor.clone()) {
                        tail = false;
                        self.todo.push(successor);
                        break;
                    }
                }
                if tail {
                    self.todo.pop();
                    return Some((label.clone(), block));
                }
            } else {
                self.todo.pop();
            }
        }
        None
    }
}

fn make_transfer<
    Initiator,
    Instruction,
    Terminator,
    F,
    InitiatorTransfer,
    InstructionTransfer,
    TerminatorTransfer,
>(
    initiator_transfer: InitiatorTransfer,
    instruction_transfer: InstructionTransfer,
    terminator_transfer: TerminatorTransfer,
) -> (
    impl Fn(&BlockOC<Instruction, Terminator>) -> F,
    impl Fn(&F, &BlockCC<Initiator, Instruction, Terminator>) -> Option<F>,
)
where
    F: Fact,
    InitiatorTransfer: Fn(&F, &Initiator) -> Option<F>,
    InstructionTransfer: Fn(&F, &Instruction) -> Option<F> + Clone,
    TerminatorTransfer: Fn(&F, &Terminator) -> Option<F> + Clone,
{
    let instruction_transfer_clone = instruction_transfer.clone();
    let terminator_transfer_clone = terminator_transfer.clone();
    (
        move |block| {
            let mut fact = F::bottom();
            for instruction in &block.instructions {
                if let Some(new_fact) = instruction_transfer(&fact, &instruction) {
                    fact = new_fact;
                }
            }
            if let Some(new_fact) = terminator_transfer(&fact, &block.terminator) {
                fact = new_fact;
            }
            fact
        },
        move |in_fact, block| {
            let mut fact = None;
            if let Some(new_fact) =
                initiator_transfer(fact.as_ref().unwrap_or(in_fact), &block.initiator)
            {
                fact = Some(new_fact);
            }
            for instruction in &block.instructions {
                if let Some(new_fact) =
                    instruction_transfer_clone(fact.as_ref().unwrap_or(in_fact), &instruction)
                {
                    fact = Some(new_fact);
                }
            }
            if let Some(new_fact) =
                terminator_transfer_clone(fact.as_ref().unwrap_or(in_fact), &block.terminator)
            {
                fact = Some(new_fact);
            }
            fact
        },
    )
}

impl<Label, Initiator, Instruction, Terminator> GraphOC<Label, Initiator, Instruction, Terminator>
where
    Terminator: Terminate<Label>,
{
    fn postorder<'a>(&'a self) -> Postorder<'a, Label, Initiator, Instruction, Terminator> {
        let mut result = Postorder {
            todo: Vec::new(),
            visited: HashSet::new(),
            graph: self,
        };
        result.todo.extend(self.entry.terminator.successors());
        result
    }

    fn analyse_forward<F, EntryTransfer, Transfer>(
        &self,
        entry_transfer: EntryTransfer,
        transfer: Transfer,
    ) -> (F, HashMap<Label, F>)
    where
        F: Fact,
        Label: Eq + Hash + Clone,
        EntryTransfer: Fn(&BlockOC<Instruction, Terminator>) -> F,
        Transfer: Fn(&F, &BlockCC<Initiator, Instruction, Terminator>) -> Option<F>,
    {
        let mut todo = VecDeque::new();
        let mut todo_set = HashSet::new();
        let mut ins = HashMap::new();
        let mut outs = HashMap::new();
        for (label, block) in self.postorder() {
            todo.push_back((label.clone(), block));
            todo_set.insert(label.clone());
        }
        let entry_out_fact = entry_transfer(&self.entry);
        for successor in self.entry.terminator.successors() {
            ins.entry(successor.clone())
                .or_insert(F::bottom())
                .join(&entry_out_fact);
        }
        while let Some((label, block)) = todo.pop_back() {
            todo_set.remove(&label);
            let in_fact = ins.entry(label.clone()).or_insert(F::bottom());
            if let Some(out_fact) = transfer(in_fact, block) {
                for successor in block.terminator.successors() {
                    ins.entry(successor.clone())
                        .or_insert(F::bottom())
                        .join(&out_fact);
                    if let Some(block) = self.labels.map.get(&successor) {
                        if todo_set.insert(successor.clone()) {
                            todo.push_front((successor, block));
                        }
                    }
                }
                outs.insert(label, out_fact);
            }
        }
        (entry_out_fact, outs)
    }
}

struct WithReplacement<Original, Replacement> {
    original: Original,
    replacement: Option<Replacement>,
}

type InitiatorWithReplacement<Label, Initiator, Instruction, Terminator> =
    WithReplacement<Initiator, GraphCO<Label, Initiator, Instruction, Terminator>>;

type InstructionWithReplacement<Label, Initiator, Instruction, Terminator> =
    WithReplacement<Instruction, GraphOO<Label, Initiator, Instruction, Terminator>>;

type TerminatorWithReplacement<Label, Initiator, Instruction, Terminator> =
    WithReplacement<Terminator, GraphOC<Label, Initiator, Instruction, Terminator>>;

impl<Label, Initiator, Instruction, Terminator>
    GraphOC<
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
    fn replace(&self) -> GraphOC<Label, Initiator, Instruction, Terminator> {
        self.and_then(
            &|label, initiator| {
                initiator
                    .replacement
                    .clone()
                    .unwrap_or_else(|| GraphCO::from((label.clone(), initiator.original.clone())))
            },
            &|instruction| {
                instruction
                    .replacement
                    .clone()
                    .unwrap_or_else(|| GraphOO::from(instruction.original.clone()))
            },
            &|terminator| {
                terminator
                    .replacement
                    .clone()
                    .unwrap_or_else(|| GraphOC::from(terminator.original.clone()))
            },
        )
    }
}
