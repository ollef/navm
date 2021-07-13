use crate::graph::*;

pub trait Fact {
    fn bottom() -> Self;
    fn join(self: &mut Self, fact: &Self);
}

pub fn make_forward_transfer<
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
    pub fn analyse_forward<F, EntryTransfer, Transfer>(
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
