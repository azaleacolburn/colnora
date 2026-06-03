use crate::instruction::Instruction;
use std::collections::HashMap;

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
