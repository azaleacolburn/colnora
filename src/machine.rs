use crate::{
    assembler::{AssemblyFile, Data},
    instruction::Instruction,
    reg::Reg,
    runnable::{Loaded, MachineState, Status, Unloaded},
    value::Value,
};
use anyhow::{Result, bail};
use std::{
    collections::HashMap,
    fs,
    io::{Read, Write},
    os::fd::FromRawFd,
    ptr,
};

type Registers = HashMap<Reg, i32>;
type Labels = HashMap<String, usize>;

#[derive(Debug, Clone)]
pub struct MachineInternal {
    stack: [u8; 480],
    registers: Registers,
    labels: Labels,
    instruction_counter: usize,
    instructions: Vec<Instruction>,
    data: HashMap<String, usize>,
}

impl MachineInternal {
    fn new() -> Box<Self> {
        Box::new(MachineInternal {
            stack: [0; 480],
            // The generic isn't relevant here
            registers: Machine::<Unloaded>::init_reg(),
            labels: HashMap::new(),
            instruction_counter: 0,
            instructions: Vec::new(),
            data: HashMap::new(),
        })
    }
}

#[derive(Debug, Clone)]
pub struct Machine<S: MachineState> {
    internal: Box<MachineInternal>,
    state: S,
}

impl<S: MachineState> Machine<S> {
    pub fn load_data(&mut self, data: Vec<(String, Data)>) {
        let mut sp = 0;

        data.iter().for_each(|(name, data)| match data {
            Data::Array(array) => {
                self.internal.data.insert(name.clone(), sp);
                array.iter().for_each(|num| {
                    self.internal.stack[sp] = *num;
                    sp += 1;
                });

                self.internal.stack[sp] = 0;
                sp += 1;
            }
            Data::Number(num) => {
                self.internal.data.insert(name.clone(), sp);

                self.internal.stack[sp] = *num as u8;
                sp += 1;
            }
        });

        println!("{:?}", self.internal.stack);
    }

    pub fn load_instructions<'a>(self, instructions: Vec<Instruction>) -> Machine<Loaded> {
        Machine {
            internal: self.internal,
            state: Loaded { instructions },
        }
    }

    pub fn load_all(self, file: AssemblyFile) -> Machine<Loaded> {
        let mut machine = self.load_instructions(file.instructions);
        machine.load_data(file.data);
        machine.load_labels();

        machine
    }
}

impl Machine<Unloaded> {
    pub fn new() -> Self {
        Machine {
            internal: MachineInternal::new(),
            state: Unloaded {},
        }
    }
}

impl<'a> Machine<Loaded> {
    pub fn load_labels(&mut self) {
        // Names are cloned to satisfy the borrow checker
        let labels: Vec<(String, usize)> = self
            .state
            .instructions
            .iter()
            .enumerate()
            .filter_map(|(i, ins)| match ins {
                Instruction::Label { name } => Some((name.to_string(), i)),
                _ => None,
            })
            .collect();

        labels.iter().for_each(|(name, i)| self.set_label(name, *i));
    }

    /// # Returns
    /// A `Status` indicating whether the simulation has halted
    pub fn step(&mut self) -> Result<Status> {
        let instruction = &self.state.instructions[self.internal.instruction_counter];
        // Determines whether to break
        if self.execute_instruction(instruction)? {
            return Ok(Status::Exit);
        }

        self.internal.instruction_counter += 1;

        Ok(Status::Continue)
    }

    pub fn run(&mut self) -> Result<()> {
        self.internal.instruction_counter = *self
            .internal
            .labels
            .get("@start")
            .expect("No @start label provided");
        while self.internal.instruction_counter < self.state.instructions.len() {
            if let Status::Exit = self.step()? {
                return Ok(());
            }
        }

        Ok(())
    }

    pub fn new(instructions: Vec<Instruction>) -> Machine<Loaded> {
        Machine {
            internal: MachineInternal::new(),
            state: Loaded { instructions },
        }
    }
}

impl<S: MachineState> Machine<S> {
    fn init_reg() -> Registers {
        let mut map: Registers = HashMap::new();

        (1..=10).map(|i| Reg::try_from(i).unwrap()).for_each(|reg| {
            map.insert(reg, 0);
        });

        map.insert(Reg::StackPtr, 0);
        map.insert(Reg::Cmp, 0);

        map
    }
}

impl<S: MachineState> Machine<S> {
    fn execute_instruction(&mut self, instruction: &Instruction) -> Result<bool> {
        // println!("Instruction: {:?}", instruction);
        match instruction {
            Instruction::Add { dest, a, b } => self.three_arg(dest, a, b, |a, b| a + b),
            Instruction::Sub { dest, a, b } => self.three_arg(dest, a, b, |a, b| a - b),
            Instruction::Mul { dest, a, b } => self.three_arg(dest, a, b, |a, b| a * b),
            Instruction::Div { dest, a, b } => self.three_arg(dest, a, b, |a, b| a / b),

            Instruction::And { dest, a, b } => self.three_arg(dest, a, b, |a, b| a & b),
            Instruction::Or { dest, a, b } => self.three_arg(dest, a, b, |a, b| a | b),
            Instruction::Xor { dest, a, b } => self.three_arg(dest, a, b, |a, b| a ^ b),
            Instruction::NAnd { dest, a, b } => self.three_arg(dest, a, b, |a, b| !(a & b)),
            Instruction::NOr { dest, a, b } => self.three_arg(dest, a, b, |a, b| !(a | b)),
            Instruction::NXor { dest, a, b } => self.three_arg(dest, a, b, |a, b| !(a ^ b)),

            Instruction::Mov { dest, a } => self.two_arg(dest, a, |a| a),
            Instruction::Not { dest, a } => self.two_arg(dest, a, |a| !a),
            Instruction::Neg { dest, a } => self.two_arg(dest, a, |a| -a),

            Instruction::Store { a, offset } => self.store(a, offset),
            Instruction::Load { dest, offset } => self.load_register(dest, offset),

            Instruction::Push { a } => self.push(a),
            Instruction::Pop { dest } => {
                println!("{:?}", self.internal.stack);
                self.pop(dest);
            }

            Instruction::Label { name: _ } => {}

            Instruction::Br { label } => self.br_label(label),
            Instruction::BrIf { label } if self.cmp() => self.br_label(label),
            Instruction::BrIf { label: _ } => {}

            Instruction::Bl { label } => self.bl_label(label),
            Instruction::BlIf { label } if self.cmp() => self.bl_label(label),
            Instruction::BlIf { label: _ } => {}

            Instruction::Ret => self.link_back(),

            Instruction::RetIf if self.cmp() => self.link_back(),
            Instruction::RetIf => {}

            Instruction::End => return Ok(true),
            Instruction::Put { a } => self.put(a),
            Instruction::SysCall => self.system_call(),
            Instruction::Assert => {
                if !self.cmp() {
                    bail!("Assertion Failed");
                }
            }

            Instruction::Eq { a, b } => self.set_cmp(a, b, |a, b| a == b),
            Instruction::NEq { a, b } => self.set_cmp(a, b, |a, b| a != b),
        };

        Ok(false)
    }

    fn two_arg(&mut self, dest: &Reg, a: &Value, pred: impl Fn(i32) -> i32) {
        self.internal
            .registers
            .insert(*dest, pred(self.evaluate(a)));
    }

    fn three_arg(&mut self, dest: &Reg, a: &Value, b: &Value, pred: impl Fn(i32, i32) -> i32) {
        let a = self.evaluate(a);
        let b = self.evaluate(b);

        self.internal.registers.insert(*dest, pred(a, b));
    }

    fn evaluate(&self, value: &Value) -> i32 {
        match value {
            Value::Reg(reg) => *self.internal.registers.get(reg).unwrap_or(&0),
            Value::Lit(n) => *n,
            Value::Deref(value) => {
                let addr = self.evaluate(value);
                self.internal.stack[addr as usize] as i32
            }
            Value::Data(name) => *self
                .internal
                .data
                .get(name)
                .expect(format!("Data ptr {} not found in .data section", name).as_str())
                as i32,
        }
    }

    // TODO
    // Maybe remove idk
    fn set_reg(&mut self, dest: &Reg, value: impl Into<i32>) {
        self.internal.registers.insert(*dest, value.into());
    }

    fn stack_pointer(&self) -> usize {
        *self
            .internal
            .registers
            .get(&Reg::StackPtr)
            .expect("Stack pointer not in registers list") as usize
    }

    fn incr_stack_pointer(&mut self) {
        self.internal
            .registers
            .entry(Reg::StackPtr)
            .and_modify(|n| *n += 1);
    }

    fn decr_stack_pointer(&mut self) {
        self.internal
            .registers
            .entry(Reg::StackPtr)
            .and_modify(|n| *n -= 1);
    }

    fn store(&mut self, a: &Value, offset: &Value) {
        let value = self.evaluate(a);
        let offset = self.evaluate(offset);

        let addr = (self.stack_pointer() as i32 + offset) as usize;

        self.internal.stack[addr] = value as u8;
    }

    fn load_register(&mut self, dest: &Reg, offset: &Value) {
        let offset = self.evaluate(offset);
        let addr = (self.stack_pointer() as i32 + offset) as usize;

        self.set_reg(dest, self.internal.stack[addr]);
    }

    fn push(&mut self, a: &Value) {
        self.internal.stack[self.stack_pointer()] = self.evaluate(a) as u8;
        self.incr_stack_pointer();
    }

    fn pop(&mut self, dest: &Reg) {
        self.decr_stack_pointer();
        self.set_reg(dest, self.internal.stack[self.stack_pointer()]);
    }

    fn link_back(&mut self) {
        self.internal.instruction_counter =
            *self.internal.registers.get(&Reg::Link).unwrap() as usize;
    }

    fn br_label(&mut self, label: &str) {
        self.internal.instruction_counter = *self.internal.labels.get(label).expect("Invalid Label")
    }

    fn bl_label(&mut self, label: &str) {
        self.set_reg(&Reg::Link, self.internal.instruction_counter as i32);
        self.br_label(label);
    }

    fn set_label(&mut self, name: &str, i: usize) {
        self.internal.labels.insert(name.to_string(), i);
    }

    fn set_cmp(&mut self, a: &Value, b: &Value, op: impl Fn(i32, i32) -> bool) {
        let a = self.evaluate(a);
        let b = self.evaluate(b);

        self.internal.registers.insert(Reg::Cmp, op(a, b) as i32);
    }

    fn cmp(&self) -> bool {
        *self.internal.registers.get(&Reg::Cmp).unwrap() == 1
    }

    fn put(&self, a: &Value) {
        let a = self.evaluate(a);

        println!("{a}");
    }

    fn system_call(&self) {
        let code = self.internal.registers.get(&Reg::A).unwrap();
        match *code {
            // Restart
            0 => {}
            1 => std::process::exit(0),
            2 => {}
            3 => self.read_sys(),
            4 => self.write_sys(),
            _ => {}
        }
    }

    fn read_sys(&self) {
        let fd = *self.internal.registers.get(&Reg::B).unwrap();
        let idx = *self.internal.registers.get(&Reg::C).unwrap() as usize;
        let len = *self.internal.registers.get(&Reg::D).unwrap() as usize;

        let buf = (&self.internal.stack[idx]) as *const u8 as *mut u8;
        let buf = unsafe { ptr::slice_from_raw_parts_mut(buf, len).as_mut().unwrap() };

        let mut file = unsafe { fs::File::from_raw_fd(fd) };
        file.read(buf).unwrap();
    }

    fn write_sys(&self) {
        let fd = *self.internal.registers.get(&Reg::B).unwrap();
        let idx = *self.internal.registers.get(&Reg::C).unwrap() as usize;
        let len = *self.internal.registers.get(&Reg::D).unwrap() as usize;

        let buf = (&self.internal.stack[idx]) as *const u8;
        let buf = unsafe { ptr::slice_from_raw_parts(buf, len).as_ref().unwrap() };

        let mut file = unsafe { fs::File::from_raw_fd(fd) };
        println!("fd {fd}");
        file.write(buf).unwrap();
    }
}
