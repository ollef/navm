use crate::graph::*;

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
    ) -> GraphCC<Label2, Initiator2, Instruction2, Terminator2>
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
