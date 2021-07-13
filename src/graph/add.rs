use crate::graph::*;

impl<Label, Initiator, Instruction, Terminator>
    Add<GraphOO<Label, Initiator, Instruction, Terminator>>
    for GraphOO<Label, Initiator, Instruction, Terminator>
where
    Label: Eq + Hash,
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
    Label: Eq + Hash,
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
    Label: Eq + Hash,
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
    Label: Eq + Hash,
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
    Label: Eq + Hash,
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
    Label: Eq + Hash,
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
