use anyhow::bail;

#[derive(Debug, Clone, Eq, Hash, PartialEq, Copy)]
pub enum Reg {
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,

    Cmp,
    StackPtr,
}

#[derive(Debug, Clone)]
pub enum Value {
    Lit(i32),
    Reg(Reg),
    Deref(Box<Value>),
}

#[derive(Debug, Clone)]
pub enum Instruction {
    Add { dest: Reg, a: Value, b: Value },
    Sub { dest: Reg, a: Value, b: Value },
    Mul { dest: Reg, a: Value, b: Value },
    Div { dest: Reg, a: Value, b: Value },

    And { dest: Reg, a: Value, b: Value },
    Or { dest: Reg, a: Value, b: Value },
    Xor { dest: Reg, a: Value, b: Value },

    NXor { dest: Reg, a: Value, b: Value },
    NAnd { dest: Reg, a: Value, b: Value },
    NOr { dest: Reg, a: Value, b: Value },

    Mov { dest: Reg, a: Value },
    Not { dest: Reg, a: Value },

    Push { a: Value },
    Pop { dest: Reg },

    Eq { a: Value, b: Value },
    NEq { a: Value, b: Value },

    Jmp { label: String },
    JmpIf { label: String },

    Label { name: String },

    Assert,
    End,
}

impl TryFrom<&str> for Instruction {
    type Error = anyhow::Error;

    fn try_from(line: &str) -> Result<Self, Self::Error> {
        let line = line.trim();
        if line.is_empty() {
            bail!("Empty Line")
        }

        let first = line.chars().nth(0).unwrap();
        let inst = match first {
            '@' => Instruction::Label {
                name: line.to_string(),
            },
            ';' => bail!("Commend"),
            _ => {
                let parts: Vec<&str> = line.trim().split(' ').collect();
                match parts.len() {
                    1 => Self::match_zero_arg_code(parts[0]),
                    2 => Self::match_one_arg_code(parts[0], parts[1]),
                    3 => Self::match_two_arg_code(parts[0], parts[1], parts[2]),
                    4 => Self::match_three_arg_code(parts[0], parts[1], parts[2], parts[3]),
                    _ => panic!("Found 4 Argument Instruction, No Such Instructions Exist"),
                }
            }
        };

        Ok(inst)
    }
}

impl Instruction {
    fn match_zero_arg_code(code: &str) -> Instruction {
        match code {
            "assert" => Instruction::Assert,
            "end" => Instruction::End,
            _ => panic!("Invalid Zero Argument Instruction: {}", code),
        }
    }

    fn match_one_arg_code(code: &str, a: &str) -> Instruction {
        match code {
            "jmp" => Instruction::Jmp {
                label: a.to_string(),
            },
            "jmpif" => Instruction::JmpIf {
                label: a.to_string(),
            },
            "push" => Instruction::Push {
                a: Value::try_from(a).expect(format!("Invalid value being pushed {}", a).as_str()),
            },
            "pop" => Instruction::Pop {
                dest: Reg::try_from(a)
                    .expect(format!("Invalid register being popped into {}", a).as_str()),
            },
            _ => panic!("Invalid code"),
        }
    }

    fn match_two_arg_code(code: &str, a: &str, b: &str) -> Instruction {
        match code {
            "mov" => {
                let dest = Reg::try_from(a).unwrap();
                let a = Value::try_from(b).unwrap();

                Instruction::Mov { dest, a }
            }
            "not" => {
                let dest = Reg::try_from(a).unwrap();
                let a = Value::try_from(b).unwrap();

                Instruction::Not { dest, a }
            }
            "eq" => {
                let a = Value::try_from(a).unwrap();
                let b = Value::try_from(b).unwrap();

                Instruction::Eq { a, b }
            }
            "neq" => {
                let a = Value::try_from(a).unwrap();
                let b = Value::try_from(b).unwrap();

                Instruction::NEq { a, b }
            }
            _ => panic!("Invalid Two Argument Instruction: {}", code),
        }
    }

    fn match_three_arg_code(code: &str, a: &str, b: &str, c: &str) -> Instruction {
        let dest = Reg::try_from(a).unwrap();
        let a = Value::try_from(b).unwrap();
        let b = Value::try_from(c).unwrap();

        match code {
            "add" => Instruction::Add { dest, a, b },
            "sub" => Instruction::Sub { dest, a, b },
            "mul" => Instruction::Mul { dest, a, b },
            "div" => Instruction::Div { dest, a, b },

            "and" => Instruction::And { dest, a, b },
            "or" => Instruction::Or { dest, a, b },
            "xor" => Instruction::Xor { dest, a, b },

            "nand" => Instruction::NAnd { dest, a, b },
            "nor" => Instruction::NOr { dest, a, b },
            "nxor" => Instruction::NXor { dest, a, b },
            _ => panic!("Invalid Two Argument Instruction: {}", code),
        }
    }
}

impl TryFrom<&str> for Value {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if value.starts_with("[") {
            let inner = Value::try_from(&value[1..value.len() - 1])?;
            assert!(value.chars().last().unwrap() == ']');

            Value::Deref(Box::new(inner));
        }
        let value = if let Ok(reg) = Reg::try_from(value) {
            Value::Reg(reg)
        } else {
            let num = value.parse::<i32>()?;
            Value::Lit(num)
        };

        Ok(value)
    }
}

impl TryFrom<&str> for Reg {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let value = match value {
            "%1" => Self::One,
            "%2" => Self::Two,
            "%3" => Self::Three,
            "%4" => Self::Four,
            "%5" => Self::Five,
            "%6" => Self::Six,
            "%7" => Self::Seven,
            "%8" => Self::Eight,
            "%9" => Self::Nine,
            "%10" => Self::Ten,
            "%cmp" => Self::Cmp,
            "%stack" => Self::StackPtr,
            _ => return Err(format!("Invalid Register Name {}", value)),
        };

        return Ok(value);
    }
}

impl TryFrom<i32> for Reg {
    type Error = String;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        let value = match value {
            1 => Self::One,
            2 => Self::Two,
            3 => Self::Three,
            4 => Self::Four,
            5 => Self::Five,
            6 => Self::Six,
            7 => Self::Seven,
            8 => Self::Eight,
            9 => Self::Nine,
            10 => Self::Ten,
            _ => return Err(format!("Invalid Register Number {}", value)),
        };

        return Ok(value);
    }
}

pub fn parse_instructions(instructions: &str) -> Vec<Instruction> {
    instructions
        .lines()
        .map(|line| line.try_into())
        .filter_map(|instruction| instruction.ok())
        .collect()
}
