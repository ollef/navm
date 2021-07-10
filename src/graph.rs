use std::collections::HashMap;
use std::collections::HashSet;
use std::hash::Hash;
use std::ops::Add;

#[derive(Clone)]
pub struct BlockOO<Instruction> {
    instructions: Vec<Instruction>,
}

pub struct BlockOC<Instruction, Terminator> {
    instructions: Vec<Instruction>,
    terminator: Terminator,
}

pub struct BlockCO<Initiator, Instruction> {
    initiator: Initiator,
    instructions: Vec<Instruction>,
}

pub struct BlockCC<Initiator, Instruction, Terminator> {
    initiator: Initiator,
    instructions: Vec<Instruction>,
    terminator: Terminator,
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
    fn new() -> BlockOO<Instruction> {
        BlockOO {
            instructions: Vec::new(),
        }
    }

    fn and_then<Label2, Initiator2, Instruction2, Terminator2, BindInstruction>(
        &self,
        bind_instruction: BindInstruction,
    ) -> GraphOO<Label2, Initiator2, Instruction2, Terminator2>
    where
        Label2: Eq + Hash + Copy,
        BindInstruction: Fn(&Instruction) -> GraphOO<Label2, Initiator2, Instruction2, Terminator2>,
    {
        self.instructions
            .iter()
            .fold(GraphOO::new(), |graph, i| graph + bind_instruction(i))
    }
}

impl<Instruction, Terminator> BlockOC<Instruction, Terminator> {
    fn and_then<Label2, Initiator2, Instruction2, Terminator2, BindInstruction, BindTerminator>(
        &self,
        bind_instruction: BindInstruction,
        bind_terminator: BindTerminator,
    ) -> GraphOC<Label2, Initiator2, Instruction2, Terminator2>
    where
        Label2: Eq + Hash + Copy,
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
    fn and_then<Label2, Initiator2, Instruction2, Terminator2, BindInitiator, BindInstruction>(
        &self,
        bind_initiator: BindInitiator,
        bind_instruction: BindInstruction,
    ) -> GraphCO<Label2, Initiator2, Instruction2, Terminator2>
    where
        Label2: Eq + Hash + Copy,
        BindInitiator: Fn(&Initiator) -> GraphCO<Label2, Initiator2, Instruction2, Terminator2>,
        BindInstruction: Fn(&Instruction) -> GraphOO<Label2, Initiator2, Instruction2, Terminator2>,
    {
        bind_initiator(&self.initiator)
            + self
                .instructions
                .iter()
                .fold(GraphOO::new(), |graph, i| graph + bind_instruction(i))
    }
}

impl<Initiator, Instruction, Terminator> BlockCC<Initiator, Instruction, Terminator> {
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
        bind_initiator: BindInitiator,
        bind_instruction: BindInstruction,
        bind_terminator: BindTerminator,
    ) -> GraphCC<Label2, Initiator2, Instruction2, Terminator2>
    where
        Label2: Eq + Hash + Copy,
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

pub struct Labels<Label, Initiator, Instruction, Terminator> {
    map: HashMap<Label, BlockCC<Initiator, Instruction, Terminator>>,
}

impl<Label, Initiator, Instruction, Terminator> Labels<Label, Initiator, Instruction, Terminator> {
    fn new() -> Labels<Label, Initiator, Instruction, Terminator> {
        Labels {
            map: HashMap::new(),
        }
    }
}

pub enum GraphOO<Label, Initiator, Instruction, Terminator> {
    Single(BlockOO<Instruction>),
    Many {
        entry: BlockOC<Instruction, Terminator>,
        labels: Labels<Label, Initiator, Instruction, Terminator>,
        exit_label: Label,
        exit: BlockCO<Initiator, Instruction>,
    },
}

pub struct GraphOC<Label, Initiator, Instruction, Terminator> {
    entry: BlockOC<Instruction, Terminator>,
    labels: Labels<Label, Initiator, Instruction, Terminator>,
}

pub struct GraphCO<Label, Initiator, Instruction, Terminator> {
    labels: Labels<Label, Initiator, Instruction, Terminator>,
    exit_label: Label,
    exit: BlockCO<Initiator, Instruction>,
}

pub struct GraphCC<Label, Initiator, Instruction, Terminator> {
    labels: Labels<Label, Initiator, Instruction, Terminator>,
}

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

impl<Label, Initiator, Instruction, Terminator>
    Add<GraphOO<Label, Initiator, Instruction, Terminator>>
    for GraphOO<Label, Initiator, Instruction, Terminator>
where
    Label: Eq + Hash + Copy,
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
    Label: Eq + Hash + Copy,
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
    Label: Eq + Hash + Copy,
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
    Label: Eq + Hash + Copy,
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
    Label: Eq + Hash + Copy,
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
    fn new() -> GraphOO<Label, Initiator, Instruction, Terminator> {
        GraphOO::Single(BlockOO::new())
    }

    fn and_then<
        Label2,
        Initiator2,
        Instruction2,
        Terminator2,
        MapLabel,
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
        Label2: Eq + Hash + Copy,
        MapLabel: Fn(&Label) -> Label2,
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
                let exit_graph = exit.and_then(
                    |initiator| bind_initiator(exit_label, initiator),
                    bind_instruction,
                );
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

impl<Label, Initiator, Instruction, Terminator> Labels<Label, Initiator, Instruction, Terminator> {
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
    ) -> Labels<Label2, Initiator2, Instruction2, Terminator2>
    where
        Label2: Eq + Hash + Copy,
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
}
