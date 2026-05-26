use std::collections::HashMap;

use crate::assembler::{Instruction, Reg, Value};

type Registers = HashMap<Reg, i32>;
type Labels = HashMap<String, usize>;

#[derive(Debug, Clone)]
pub struct Machine {
    stack: [i32; 480],
    registers: Registers,
    labels: Labels,
    instruction_counter: usize,
}

impl Machine {
    pub fn new() -> Self {
        Machine {
            stack: [0; 480],
            registers: Self::init_reg(),
            labels: HashMap::new(),
            instruction_counter: 0,
        }
    }
    pub fn run(&mut self, instructions: &[Instruction]) {
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
            self.execute_instruction(instruction);

            self.instruction_counter += 1;
        }
    }

    fn execute_instruction(&mut self, instruction: &Instruction) {
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

            Instruction::Label { name: _ } => {}
            Instruction::Jmp { label } => self.goto_label(label),

            Instruction::JmpIf { label } if self.cmp() => self.goto_label(label),
            Instruction::JmpIf { label: _ } => {}

            Instruction::Assert => assert!(self.cmp()),
            Instruction::Eq { a, b } => self.set_cmp(a, b, |a, b| a == b),
            Instruction::NEq { a, b } => self.set_cmp(a, b, |a, b| a != b),
        };
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
