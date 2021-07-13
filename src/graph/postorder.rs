use crate::graph::*;

pub struct Postorder<'a, Label, Initiator, Instruction, Terminator> {
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
    pub fn postorder<'a>(&'a self) -> Postorder<'a, Label, Initiator, Instruction, Terminator> {
        let mut result = Postorder {
            todo: Vec::new(),
            visited: HashSet::new(),
            graph: self,
        };
        result.todo.extend(self.entry.terminator.successors());
        result
    }
}
