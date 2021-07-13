use crate::block::*;

impl<Instruction> BlockOO<Instruction> {
    pub fn and_then<Label2, Initiator2, Instruction2, Terminator2, BindInstruction>(
        &self,
        bind_instruction: BindInstruction,
    ) -> GraphOO<Label2, Initiator2, Instruction2, Terminator2>
    where
        Label2: Eq + Hash,
        BindInstruction: Fn(&Instruction) -> GraphOO<Label2, Initiator2, Instruction2, Terminator2>,
    {
        self.instructions
            .iter()
            .fold(GraphOO::new(), |graph, i| graph + bind_instruction(i))
    }

    pub fn and_then_into<Label2, Initiator2, Instruction2, Terminator2, BindInstruction>(
        self,
        bind_instruction: BindInstruction,
    ) -> GraphOO<Label2, Initiator2, Instruction2, Terminator2>
    where
        Label2: Eq + Hash,
        BindInstruction: Fn(Instruction) -> GraphOO<Label2, Initiator2, Instruction2, Terminator2>,
    {
        self.instructions
            .into_iter()
            .fold(GraphOO::new(), |graph, i| graph + bind_instruction(i))
    }
}

impl<Instruction, Terminator> BlockOC<Instruction, Terminator> {
    pub fn and_then<
        Label2,
        Initiator2,
        Instruction2,
        Terminator2,
        BindInstruction,
        BindTerminator,
    >(
        &self,
        bind_instruction: BindInstruction,
        bind_terminator: BindTerminator,
    ) -> GraphOC<Label2, Initiator2, Instruction2, Terminator2>
    where
        Label2: Eq + Hash,
        BindInstruction: Fn(&Instruction) -> GraphOO<Label2, Initiator2, Instruction2, Terminator2>,
        BindTerminator: Fn(&Terminator) -> GraphOC<Label2, Initiator2, Instruction2, Terminator2>,
    {
        self.instructions
            .iter()
            .fold(GraphOO::new(), |graph, i| graph + bind_instruction(i))
            + bind_terminator(&self.terminator)
    }

    pub fn and_then_into<
        Label2,
        Initiator2,
        Instruction2,
        Terminator2,
        BindInstruction,
        BindTerminator,
    >(
        self,
        bind_instruction: BindInstruction,
        bind_terminator: BindTerminator,
    ) -> GraphOC<Label2, Initiator2, Instruction2, Terminator2>
    where
        Label2: Eq + Hash,
        BindInstruction: Fn(Instruction) -> GraphOO<Label2, Initiator2, Instruction2, Terminator2>,
        BindTerminator: Fn(Terminator) -> GraphOC<Label2, Initiator2, Instruction2, Terminator2>,
    {
        self.instructions
            .into_iter()
            .fold(GraphOO::new(), |graph, i| graph + bind_instruction(i))
            + bind_terminator(self.terminator)
    }
}

impl<Initiator, Instruction> BlockCO<Initiator, Instruction> {
    pub fn and_then<Label2, Initiator2, Instruction2, Terminator2, BindInitiator, BindInstruction>(
        &self,
        bind_initiator: BindInitiator,
        bind_instruction: BindInstruction,
    ) -> GraphCO<Label2, Initiator2, Instruction2, Terminator2>
    where
        Label2: Eq + Hash,
        BindInitiator: FnOnce(&Initiator) -> GraphCO<Label2, Initiator2, Instruction2, Terminator2>,
        BindInstruction: Fn(&Instruction) -> GraphOO<Label2, Initiator2, Instruction2, Terminator2>,
    {
        bind_initiator(&self.initiator)
            + self
                .instructions
                .iter()
                .fold(GraphOO::new(), |graph, i| graph + bind_instruction(i))
    }

    pub fn and_then_into<
        Label2,
        Initiator2,
        Instruction2,
        Terminator2,
        BindInitiator,
        BindInstruction,
    >(
        self,
        bind_initiator: BindInitiator,
        bind_instruction: BindInstruction,
    ) -> GraphCO<Label2, Initiator2, Instruction2, Terminator2>
    where
        Label2: Eq + Hash,
        BindInitiator: FnOnce(Initiator) -> GraphCO<Label2, Initiator2, Instruction2, Terminator2>,
        BindInstruction: Fn(Instruction) -> GraphOO<Label2, Initiator2, Instruction2, Terminator2>,
    {
        bind_initiator(self.initiator)
            + self
                .instructions
                .into_iter()
                .fold(GraphOO::new(), |graph, i| graph + bind_instruction(i))
    }
}

impl<Initiator, Instruction, Terminator> BlockCC<Initiator, Instruction, Terminator> {
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
        bind_initiator: BindInitiator,
        bind_instruction: BindInstruction,
        bind_terminator: BindTerminator,
    ) -> GraphCC<Label2, Initiator2, Instruction2, Terminator2>
    where
        Label2: Eq + Hash,
        BindInitiator: FnOnce(&Initiator) -> GraphCO<Label2, Initiator2, Instruction2, Terminator2>,
        BindInstruction: Fn(&Instruction) -> GraphOO<Label2, Initiator2, Instruction2, Terminator2>,
        BindTerminator:
            FnOnce(&Terminator) -> GraphOC<Label2, Initiator2, Instruction2, Terminator2>,
    {
        bind_initiator(&self.initiator)
            + self
                .instructions
                .iter()
                .fold(GraphOO::new(), |graph, i| graph + bind_instruction(i))
            + bind_terminator(&self.terminator)
    }

    pub fn and_then_into<
        Label2,
        Initiator2,
        Instruction2,
        Terminator2,
        BindInitiator,
        BindInstruction,
        BindTerminator,
    >(
        self,
        bind_initiator: BindInitiator,
        bind_instruction: BindInstruction,
        bind_terminator: BindTerminator,
    ) -> GraphCC<Label2, Initiator2, Instruction2, Terminator2>
    where
        Label2: Eq + Hash,
        BindInitiator: FnOnce(Initiator) -> GraphCO<Label2, Initiator2, Instruction2, Terminator2>,
        BindInstruction: Fn(Instruction) -> GraphOO<Label2, Initiator2, Instruction2, Terminator2>,
        BindTerminator:
            FnOnce(Terminator) -> GraphOC<Label2, Initiator2, Instruction2, Terminator2>,
    {
        bind_initiator(self.initiator)
            + self
                .instructions
                .into_iter()
                .fold(GraphOO::new(), |graph, i| graph + bind_instruction(i))
            + bind_terminator(self.terminator)
    }
}
