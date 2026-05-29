use crate::assembler::{Data, Instruction, Reg, Value};
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
pub struct Machine {
    stack: [u8; 480],
    registers: Registers,
    labels: Labels,
    instruction_counter: usize,
    data: HashMap<String, usize>,
}

impl Machine {
    pub fn new() -> Self {
        Machine {
            stack: [0; 480],
            registers: Self::init_reg(),
            labels: HashMap::new(),
            instruction_counter: 0,
            data: HashMap::new(),
        }
    }

    pub fn load(&mut self, data: &[(String, Data)]) {
        let mut sp = 0;

        data.iter().for_each(|(name, data)| match data {
            Data::Array(array) => {
                self.data.insert(name.clone(), sp);
                array.iter().for_each(|num| {
                    self.stack[sp] = *num;
                    sp += 1;
                });

                self.stack[sp] = 0;
                sp += 1;
            }
            Data::Number(num) => {
                self.data.insert(name.clone(), sp);

                self.stack[sp] = *num as u8;
                sp += 1;
            }
        });

        println!("{:?}", self.stack);
    }

    pub fn run(&mut self, instructions: &[Instruction]) -> Result<()> {
        instructions
            .iter()
            .enumerate()
            .filter_map(|(i, ins)| match ins {
                Instruction::Label { name } => Some((name, i)),
                _ => None,
            })
            .for_each(|(name, i)| self.set_label(name, i));

        self.instruction_counter = *self.labels.get("@start").expect("No @start label provided");
        while self.instruction_counter < instructions.len() {
            let instruction = &instructions[self.instruction_counter];
            // Determines whether to break
            if self.execute_instruction(instruction)? {
                return Ok(());
            }

            self.instruction_counter += 1;
        }

        Ok(())
    }

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
                println!("{:?}", self.stack);
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
        self.registers.insert(*dest, pred(self.evaluate(a)));
    }

    fn three_arg(&mut self, dest: &Reg, a: &Value, b: &Value, pred: impl Fn(i32, i32) -> i32) {
        let a = self.evaluate(a);
        let b = self.evaluate(b);

        self.registers.insert(*dest, pred(a, b));
    }

    fn evaluate(&self, value: &Value) -> i32 {
        match value {
            Value::Reg(reg) => *self.registers.get(reg).unwrap_or(&0),
            Value::Lit(n) => *n,
            Value::Deref(value) => {
                let addr = self.evaluate(value);
                self.stack[addr as usize] as i32
            }
            Value::Data(name) => *self
                .data
                .get(name)
                .expect(format!("Data ptr {} not found in .data section", name).as_str())
                as i32,
        }
    }

    // TODO
    // Maybe remove idk
    fn set_reg(&mut self, dest: &Reg, value: impl Into<i32>) {
        self.registers.insert(*dest, value.into());
    }

    fn stack_pointer(&self) -> usize {
        *self
            .registers
            .get(&Reg::StackPtr)
            .expect("Stack pointer not in registers list") as usize
    }

    fn incr_stack_pointer(&mut self) {
        self.registers.entry(Reg::StackPtr).and_modify(|n| *n += 1);
    }

    fn decr_stack_pointer(&mut self) {
        self.registers.entry(Reg::StackPtr).and_modify(|n| *n -= 1);
    }

    fn store(&mut self, a: &Value, offset: &Value) {
        let value = self.evaluate(a);
        let offset = self.evaluate(offset);

        let addr = (self.stack_pointer() as i32 + offset) as usize;

        self.stack[addr] = value as u8;
    }

    fn load_register(&mut self, dest: &Reg, offset: &Value) {
        let offset = self.evaluate(offset);
        let addr = (self.stack_pointer() as i32 + offset) as usize;

        self.set_reg(dest, self.stack[addr]);
    }

    fn push(&mut self, a: &Value) {
        self.stack[self.stack_pointer()] = self.evaluate(a) as u8;
        self.incr_stack_pointer();
    }

    fn pop(&mut self, dest: &Reg) {
        self.decr_stack_pointer();
        self.set_reg(dest, self.stack[self.stack_pointer()]);
    }

    fn link_back(&mut self) {
        self.instruction_counter = *self.registers.get(&Reg::Link).unwrap() as usize;
    }

    fn br_label(&mut self, label: &str) {
        self.instruction_counter = *self.labels.get(label).expect("Invalid Label")
    }

    fn bl_label(&mut self, label: &str) {
        self.set_reg(&Reg::Link, self.instruction_counter as i32);
        self.br_label(label);
    }

    fn set_label(&mut self, name: &str, i: usize) {
        self.labels.insert(name.to_string(), i);
    }

    fn set_cmp(&mut self, a: &Value, b: &Value, op: impl Fn(i32, i32) -> bool) {
        let a = self.evaluate(a);
        let b = self.evaluate(b);

        self.registers.insert(Reg::Cmp, op(a, b) as i32);
    }

    fn cmp(&self) -> bool {
        *self.registers.get(&Reg::Cmp).unwrap() == 1
    }

    fn put(&self, a: &Value) {
        let a = self.evaluate(a);

        println!("{a}");
    }

    fn system_call(&self) {
        let code = self.registers.get(&Reg::A).unwrap();
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
        let fd = *self.registers.get(&Reg::B).unwrap();
        let idx = *self.registers.get(&Reg::C).unwrap() as usize;
        let len = *self.registers.get(&Reg::D).unwrap() as usize;

        let buf = (&self.stack[idx]) as *const u8 as *mut u8;
        let buf = unsafe { ptr::slice_from_raw_parts_mut(buf, len).as_mut().unwrap() };

        let mut file = unsafe { fs::File::from_raw_fd(fd) };
        file.read(buf).unwrap();
    }

    fn write_sys(&self) {
        println!("{:?}", self);
        let fd = *self.registers.get(&Reg::B).unwrap();
        let idx = *self.registers.get(&Reg::C).unwrap() as usize;
        let len = *self.registers.get(&Reg::D).unwrap() as usize;

        let buf = (&self.stack[idx]) as *const u8;
        let buf = unsafe { ptr::slice_from_raw_parts(buf, len).as_ref().unwrap() };

        let mut file = unsafe { fs::File::from_raw_fd(fd) };
        println!("fd {fd}");
        file.write(buf).unwrap();
    }
}

impl Machine {
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
