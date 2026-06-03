use std::fmt::Display;

use anyhow::bail;

use crate::{reg::Reg, value::Value};

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
    Neg { dest: Reg, a: Value },

    Store { a: Value, offset: Value },
    Load { dest: Reg, offset: Value },

    // Should be sugar for:
    // `str a 0`
    // `add %sp %sp 1`
    // or smth like that
    Push { a: Value },
    Pop { dest: Reg },

    Eq { a: Value, b: Value },
    NEq { a: Value, b: Value },

    // Branch without link
    Br { label: String },
    BrIf { label: String },

    // Branch with link
    Bl { label: String },
    BlIf { label: String },

    Label { name: String },
    Put { a: Value },

    Assert,
    Ret,
    RetIf,
    SysCall,
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
            '.' => bail!("Section Header"),
            ';' => bail!("Commend"),
            _ => {
                let comment_removal: Vec<&str> = line.trim().split(';').collect();
                let parts: Vec<&str> = comment_removal[0].trim().split(' ').collect();
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

/// Format Instruction Macro
/// The first argument is the name of the instruction (as it should be formatted)
/// All subsequent arguments are properties that must implement `Into<&str>` and
/// will be formatted along with the name separated by spaces
macro_rules! fmt_inst {
    ($name:expr, $( $prop:expr ),*) => {
        {
        let mut str = $name.to_string();
        $(
            str.push_str(Into::<String>::into($prop).as_str());
        )*
        str
        }
    };
}

impl Display for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            Instruction::End => "end".to_string(),
            Instruction::Assert => "assert".to_string(),
            Instruction::Ret => "ret".to_string(),
            Instruction::RetIf => "retif".to_string(),
            Instruction::SysCall => "sys".to_string(),

            Instruction::Label { name } => format!("@{}", name),

            Instruction::And { dest, a, b } => fmt_inst!("and", dest, a, b),
            Instruction::Or { dest, a, b } => fmt_inst!("or", dest, a, b),
            Instruction::Xor { dest, a, b } => fmt_inst!("xor", dest, a, b),
            Instruction::NAnd { dest, a, b } => fmt_inst!("nand", dest, a, b),
            Instruction::NOr { dest, a, b } => fmt_inst!("nor", dest, a, b),
            Instruction::NXor { dest, a, b } => fmt_inst!("nxor", dest, a, b),

            Instruction::Add { dest, a, b } => fmt_inst!("add", dest, a, b),
            Instruction::Sub { dest, a, b } => fmt_inst!("sub", dest, a, b),
            Instruction::Mul { dest, a, b } => fmt_inst!("mul", dest, a, b),
            Instruction::Div { dest, a, b } => fmt_inst!("div", dest, a, b),

            Instruction::Br { label } => fmt_inst!("br", label),
            Instruction::BrIf { label } => fmt_inst!("brif", label),
            Instruction::Bl { label } => fmt_inst!("bl", label),
            Instruction::BlIf { label } => fmt_inst!("blif", label),

            Instruction::Mov { dest, a } => fmt_inst!("mov", dest, a),
            Instruction::Not { dest, a } => fmt_inst!("not", dest, a),
            Instruction::Neg { dest, a } => fmt_inst!("neg", dest, a),

            Instruction::Load { dest, offset } => fmt_inst!("ldr", dest, offset),
            Instruction::Store { a, offset } => fmt_inst!("str", a, offset),
            Instruction::Push { a } => fmt_inst!("push", a),
            Instruction::Pop { dest } => fmt_inst!("pop", dest),
            Instruction::Put { a } => fmt_inst!("pub", a),

            Instruction::Eq { a, b } => fmt_inst!("eq", a, b),
            Instruction::NEq { a, b } => fmt_inst!("neq", a, b),
        };

        f.write_str(&str)
    }
}

impl Instruction {
    fn match_zero_arg_code(code: &str) -> Instruction {
        match code {
            "assert" => Instruction::Assert,
            "end" => Instruction::End,
            "ret" => Instruction::Ret,
            "retif" => Instruction::RetIf,
            "sys" => Instruction::SysCall,
            _ => panic!("Invalid Zero Argument Instruction: {}", code),
        }
    }

    fn match_one_arg_code(code: &str, a: &str) -> Instruction {
        match code {
            "bl" => Instruction::Bl {
                label: a.to_string(),
            },
            "blif" => Instruction::BlIf {
                label: a.to_string(),
            },
            "br" => Instruction::Br {
                label: a.to_string(),
            },
            "brif" => Instruction::BrIf {
                label: a.to_string(),
            },
            "push" => Instruction::Push {
                a: Value::try_from(a).expect(format!("Invalid value being pushed {}", a).as_str()),
            },
            "pop" => Instruction::Pop {
                dest: Reg::try_from(a)
                    .expect(format!("Invalid register being popped into {}", a).as_str()),
            },
            "put" => Instruction::Put {
                a: Value::try_from(a).expect(format!("Invalid value being put {}", a).as_str()),
            },
            _ => panic!("Invalid code"),
        }
    }

    fn match_two_arg_code(code: &str, a: &str, b: &str) -> Instruction {
        macro_rules! generate {
            ($t:ty) => {
                |a: &str, b: &str| {
                    let a = <$t>::try_from(a).unwrap();
                    let b = Value::try_from(b).unwrap();

                    (a, b)
                }
            };
        }

        let reg_val = generate!(Reg);
        let val_val = generate!(Value);

        match code {
            "mov" => {
                let (dest, a) = reg_val(a, b);
                Instruction::Mov { dest, a }
            }
            "not" => {
                let (dest, a) = reg_val(a, b);
                Instruction::Not { dest, a }
            }

            "neg" => {
                let (dest, a) = reg_val(a, b);
                Instruction::Neg { dest, a }
            }
            "eq" => {
                let (a, b) = val_val(a, b);
                Instruction::Eq { a, b }
            }
            "neq" => {
                let (a, b) = val_val(a, b);
                Instruction::NEq { a, b }
            }

            "str" => {
                let (a, offset) = val_val(a, b);
                Instruction::Store { a, offset }
            }
            "ldr" => {
                let (dest, offset) = reg_val(a, b);
                Instruction::Load { dest, offset }
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
