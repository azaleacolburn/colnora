use std::collections::HashMap;

use anyhow::bail;

#[derive(Debug, Clone, Eq, Hash, PartialEq, Copy)]
pub enum Reg {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    I,
    J,

    Cmp,
    Link,
    StackPtr,
}

#[derive(Debug, Clone)]
pub enum Value {
    Lit(i32),
    Reg(Reg),
    Data(String),
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
    Neg { dest: Reg, a: Value },

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
                a: Value::try_from(a).expect(format!("Invalid value being pushed {}", a).as_str()),
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
            "neg" => {
                let dest = Reg::try_from(a).unwrap();
                let a = Value::try_from(b).unwrap();

                Instruction::Neg { dest, a }
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
        let value = if value.starts_with("[") {
            let inner = Value::try_from(&value[1..value.len() - 1])?;
            assert!(value.chars().last().unwrap() == ']');

            Value::Deref(Box::new(inner))
        } else if let Ok(reg) = Reg::try_from(value) {
            Value::Reg(reg)
        } else if let Ok(num) = value.parse::<i32>() {
            Value::Lit(num)
        } else {
            Value::Data(value.to_string())
        };

        Ok(value)
    }
}

impl TryFrom<&str> for Reg {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let value = match value {
            "%a" => Self::A,
            "%b" => Self::B,
            "%c" => Self::C,
            "%d" => Self::D,
            "%e" => Self::E,
            "%f" => Self::F,
            "%g" => Self::G,
            "%h" => Self::H,
            "%i" => Self::I,
            "%j" => Self::J,
            "%cmp" => Self::Cmp,
            "%lnk" => Self::Link,
            "%sp" => Self::StackPtr,
            _ => return Err(format!("Invalid Register Name {}", value)),
        };

        return Ok(value);
    }
}

impl TryFrom<i32> for Reg {
    type Error = String;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        let value = match value {
            1 => Self::A,
            2 => Self::B,
            3 => Self::C,
            4 => Self::D,
            5 => Self::E,
            6 => Self::F,
            7 => Self::G,
            8 => Self::H,
            9 => Self::I,
            10 => Self::J,
            _ => return Err(format!("Invalid Register Number {}", value)),
        };

        return Ok(value);
    }
}

#[derive(Debug, Clone)]
pub enum Data {
    Array(Vec<u8>),
    Number(i32),
}

pub struct AssemblyFile {
    pub instructions: Vec<Instruction>,
    pub data: Vec<(String, Data)>,
}

pub fn parse_file(file_text: &str) -> AssemblyFile {
    let sections = split_file_by_section(file_text);

    println!("{:?}", sections);

    let instructions = sections.get("text").expect("No .text section provided");
    let instructions = parse_instructions(instructions);

    let data = sections.get("data").unwrap_or(&"");
    let data = parse_data(data);

    AssemblyFile { instructions, data }
}

pub fn split_file_by_section(file_text: &str) -> HashMap<&str, &str> {
    let mut sections: HashMap<&str, &str> = HashMap::new();

    file_text
        .split('.')
        .filter_map(|section| section.split_once('\n'))
        .for_each(|(name, section)| {
            sections.insert(name, section);
        });

    sections
}

pub fn parse_instructions(instructions: &str) -> Vec<Instruction> {
    instructions
        .lines()
        .map(|line| line.try_into())
        .filter_map(|instruction| instruction.ok())
        .collect()
}

fn parse_string(value: &str) -> Data {
    Data::Array(value[1..value.len() - 1].bytes().collect::<Vec<u8>>())
}

fn parse_char(value: &str) -> Data {
    let c = &value[1..value.len() - 1];
    assert!(c.len() == 1);
    let c = c.as_bytes()[0];

    Data::Number(c as i32)
}

fn parse_num(value: &str, name: &str) -> Data {
    let num = value
        .parse::<i32>()
        .expect(format!("Invalid Number As Value For Data Section Object {}", name).as_str());

    Data::Number(num)
}

pub fn parse_data(data: &str) -> Vec<(String, Data)> {
    data.lines()
        .filter(|line| !line.is_empty())
        .map(|line| line.split(':').collect())
        .map(|split: Vec<&str>| (split[0].to_string(), split[1].trim()))
        .map(|(name, value)| {
            let value = match value
                .chars()
                .nth(0)
                .expect(format!("No Value For Data Section Object {}", name).as_str())
            {
                '"' => parse_string(value),
                '\'' => parse_char(value),
                n if n.is_numeric() => parse_num(value, &name),
                _ => panic!("Invalid Value For Data Section Object {}", name),
            };

            (name, value)
        })
        .collect()
}
