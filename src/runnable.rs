use crate::{assembler::Data, instruction::Instruction};
use anyhow::Result;

pub enum Status {
    Continue,
    Exit,
}

pub trait Runnable<'a> {
    fn load_data(&mut self, data: &[(String, Data)]);
    fn load_instructions(&mut self, instructions: &'a [Instruction]);
    fn load_labels(&mut self);

    fn load_all(&mut self, data: &[(String, Data)], instructions: &'a [Instruction]) {
        self.load_data(data);
        self.load_instructions(instructions);
        self.load_labels();
    }

    fn run(&mut self) -> Result<()>;

    fn step(&mut self) -> Result<Status>;
}

#[derive(Debug, Clone)]
pub struct Loaded {
    pub instructions: Vec<Instruction>,
}
#[derive(Debug, Clone)]
pub struct Unloaded {}

pub trait MachineState {}

impl MachineState for Loaded {}
impl MachineState for Unloaded {}
