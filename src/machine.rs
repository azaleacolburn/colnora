use crate::assembler::{Instruction, Reg, Value};
use anyhow::{Result, bail};
use std::collections::HashMap;

type Registers = HashMap<Reg, i32>;
type Labels = HashMap<String, usize>;

#[derive(Debug, Clone)]
pub struct Machine {
    stack: [i32; 480],
    registers: Registers,
    labels: Labels,
    instruction_counter: usize,
    stop: bool,
}

impl Machine {
    pub fn new() -> Self {
        Machine {
            stack: [0; 480],
            registers: Self::init_reg(),
            labels: HashMap::new(),
            instruction_counter: 0,
            stop: false,
        }
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

        self.instruction_counter = 0;
        while self.instruction_counter < instructions.len() {
            let instruction = &instructions[self.instruction_counter];
            self.execute_instruction(instruction)?;
            if self.stop {
                return Ok(());
            }

            self.instruction_counter += 1;
        }

        Ok(())
    }

    fn execute_instruction(&mut self, instruction: &Instruction) -> Result<()> {
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

            Instruction::Push { a } => self.push(a),
            Instruction::Pop { dest } => self.pop(dest),

            Instruction::Label { name: _ } => {}
            Instruction::Jmp { label } => self.goto_label(label),

            Instruction::JmpIf { label } if self.cmp() => self.goto_label(label),
            Instruction::JmpIf { label: _ } => {}

            Instruction::End => self.stop = true,
            Instruction::Assert => {
                if !self.cmp() {
                    bail!("Assertion Failed");
                }
            }
            Instruction::Eq { a, b } => self.set_cmp(a, b, |a, b| a == b),
            Instruction::NEq { a, b } => self.set_cmp(a, b, |a, b| a != b),
        };

        Ok(())
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
                self.stack[addr as usize]
            }
        }
    }

    // TODO
    // Maybe remove idk
    fn set_reg(&mut self, dest: &Reg, value: i32) {
        self.registers.insert(*dest, value);
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

    fn push(&mut self, a: &Value) {
        self.stack[self.stack_pointer()] = self.evaluate(a);
        self.incr_stack_pointer();
    }

    fn pop(&mut self, dest: &Reg) {
        self.decr_stack_pointer();
        self.set_reg(dest, self.stack[self.stack_pointer()]);
    }

    fn goto_label(&mut self, label: &str) {
        self.instruction_counter = *self.labels.get(label).expect("Invalid Label")
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
