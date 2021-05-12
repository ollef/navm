use std::collections::HashMap;
use std::collections::HashSet;
use std::ops::Add;

#[derive(Clone)]
pub struct BlockO<I> {
    instructions: Vec<I>,
}

impl<I> From<I> for BlockO<I> {
    fn from(i: I) -> BlockO<I> {
        BlockO {
            instructions: vec![i],
        }
    }
}

impl<I> BlockO<I> {
    fn new() -> BlockO<I> {
        BlockO {
            instructions: Vec::new(),
        }
    }

    fn and_then<T, S, J, F>(&self, f: F) -> GraphOO<J, S>
    where
        F: Fn(&I) -> GraphOO<J, S>,
    {
        self.instructions
            .iter()
            .fold(GraphOO::<J, S>::new(), |graph, i| graph + f(i))
    }
}

pub struct BlockC<I, T> {
    instructions: Vec<I>,
    terminator: T,
}

impl<I, T> BlockC<I, T> {
    fn and_then<J, U, F, G>(&self, f: F, g: G) -> GraphOC<J, U>
    where
        F: Fn(&I) -> GraphOO<J, U>,
        G: Fn(&T) -> GraphOC<J, U>,
    {
        self.instructions
            .iter()
            .fold(GraphOO::<J, U>::new(), |graph, i| graph + f(i))
            + g(&self.terminator)
    }
}

impl<I, T> From<T> for BlockC<I, T> {
    fn from(t: T) -> BlockC<I, T> {
        BlockC {
            instructions: vec![],
            terminator: t,
        }
    }
}

impl<I> Add<BlockO<I>> for BlockO<I> {
    type Output = BlockO<I>;
    fn add(mut self, mut rhs: BlockO<I>) -> BlockO<I> {
        self.instructions.append(&mut rhs.instructions);
        self
    }
}

impl<I, T> Add<BlockC<I, T>> for BlockO<I> {
    type Output = BlockC<I, T>;
    fn add(mut self, mut rhs: BlockC<I, T>) -> BlockC<I, T> {
        self.instructions.append(&mut rhs.instructions);
        BlockC {
            instructions: self.instructions,
            terminator: rhs.terminator,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
pub struct Label {
    unique: i32,
}

pub struct Labels<I, T> {
    map: HashMap<Label, BlockC<I, T>>,
}

impl<I, T> Labels<I, T> {
    fn new() -> Labels<I, T> {
        Labels {
            map: HashMap::new(),
        }
    }
}

pub enum GraphOO<I, T> {
    Single(BlockO<I>),
    Many {
        entry: BlockC<I, T>,
        labels: Labels<I, T>,
        exit_label: Label,
        exit: BlockO<I>,
    },
}

impl<I, T> GraphOO<I, T> {
    fn new() -> GraphOO<I, T> {
        GraphOO::Single(BlockO::<I>::new())
    }

    fn and_then<J, U, F, G>(&self, f: &F, g: &G) -> GraphOO<J, U>
    where
        F: Fn(&I) -> GraphOO<J, U>,
        G: Fn(&T) -> GraphOC<J, U>,
    {
        match self {
            GraphOO::Single(block) => block.and_then::<T, _, _, _>(f),
            GraphOO::Many {
                entry,
                labels,
                exit_label,
                exit,
            } => {
                let entry_graph = entry.and_then(f, g);
                let entry = entry_graph.entry;
                let mut labels = labels.and_then(f, g);
                let exit_graph = exit.and_then::<T, _, _, _>(f);
                labels.map.extend(entry_graph.labels.map);
                match exit_graph {
                    GraphOO::Single(exit) => GraphOO::Many {
                        entry,
                        labels,
                        exit_label: *exit_label,
                        exit,
                    },
                    GraphOO::Many {
                        entry: exit_entry,
                        labels: exit_labels,
                        exit_label: exit_exit_label,
                        exit: exit_exit,
                    } => {
                        labels.map.extend(exit_labels.map);
                        labels.map.insert(*exit_label, exit_entry);
                        GraphOO::Many {
                            entry,
                            labels,
                            exit_label: exit_exit_label,
                            exit: exit_exit,
                        }
                    }
                }
            }
        }
    }
}

impl<I, T> Labels<I, T> {
    fn and_then<J, U, F, G>(&self, f: &F, g: &G) -> Labels<J, U>
    where
        F: Fn(&I) -> GraphOO<J, U>,
        G: Fn(&T) -> GraphOC<J, U>,
    {
        let mut result = Labels::new();
        for (label, block) in self.map.iter() {
            let graph = block.and_then(f, g);
            result.map.insert(*label, graph.entry);
            result.map.extend(graph.labels.map);
        }
        result
    }
}

pub struct GraphOC<I, T> {
    entry: BlockC<I, T>,
    labels: Labels<I, T>,
}

pub struct GraphCO<I, T> {
    labels: Labels<I, T>,
    exit_label: Label,
    exit: BlockO<I>,
}

pub struct GraphCC<I, T> {
    labels: Labels<I, T>,
}

impl<I, T> From<I> for GraphOO<I, T> {
    fn from(i: I) -> GraphOO<I, T> {
        GraphOO::Single(BlockO::<I>::from(i))
    }
}

impl<I, T> From<T> for GraphOC<I, T> {
    fn from(t: T) -> GraphOC<I, T> {
        GraphOC {
            entry: BlockC::<I, T>::from(t),
            labels: Labels::new(),
        }
    }
}

impl<I, T> Add<GraphOO<I, T>> for GraphOO<I, T> {
    type Output = GraphOO<I, T>;
    fn add(self, other: GraphOO<I, T>) -> GraphOO<I, T> {
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

impl<I, T> Add<GraphOC<I, T>> for GraphOO<I, T> {
    type Output = GraphOC<I, T>;
    fn add(self, other: GraphOC<I, T>) -> GraphOC<I, T> {
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

impl<I, T> Add<GraphOO<I, T>> for GraphCO<I, T> {
    type Output = GraphCO<I, T>;
    fn add(mut self, other: GraphOO<I, T>) -> GraphCO<I, T> {
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

impl<I, T> Add<GraphOC<I, T>> for GraphCO<I, T> {
    type Output = GraphCC<I, T>;
    fn add(mut self, other: GraphOC<I, T>) -> GraphCC<I, T> {
        self.labels.map.extend(other.labels.map);
        self.labels
            .map
            .insert(self.exit_label, self.exit + other.entry);
        GraphCC {
            labels: self.labels,
        }
    }
}

impl<I, T> Add<BlockO<I>> for GraphCO<I, T> {
    type Output = GraphCO<I, T>;
    fn add(self, other: BlockO<I>) -> GraphCO<I, T> {
        GraphCO {
            labels: self.labels,
            exit_label: self.exit_label,
            exit: self.exit + other,
        }
    }
}

impl<I, T> Add<BlockO<I>> for GraphOO<I, T> {
    type Output = GraphOO<I, T>;
    fn add(self, other: BlockO<I>) -> GraphOO<I, T> {
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

impl<I, T> Add<GraphOO<I, T>> for BlockO<I> {
    type Output = GraphOO<I, T>;
    fn add(self, other: GraphOO<I, T>) -> GraphOO<I, T> {
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

impl<I, T> Add<GraphOC<I, T>> for BlockO<I> {
    type Output = GraphOC<I, T>;
    fn add(self, other: GraphOC<I, T>) -> GraphOC<I, T> {
        GraphOC {
            entry: self + other.entry,
            labels: other.labels,
        }
    }
}

impl<I, T> Add<BlockC<I, T>> for GraphOO<I, T> {
    type Output = GraphOC<I, T>;
    fn add(self, other: BlockC<I, T>) -> GraphOC<I, T> {
        match self {
            GraphOO::Single(block) => GraphOC {
                entry: block + other,
                labels: Labels::<I, T>::new(),
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

pub trait Terminate {
    fn successors(self: &Self) -> HashSet<Label>;
}
