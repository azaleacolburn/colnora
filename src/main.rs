use std::fs::read_to_string;

use crate::machine::Machine;

pub mod assembler;
pub mod machine;

fn main() -> anyhow::Result<()> {
    let instructions_str = read_to_string("tests/test.s")?;
    let instructions = assembler::parse_instructions(&instructions_str);
    println!("{:?}", instructions);

    let mut machine = Machine::new();
    machine.run(&instructions);

    println!("{:?}", machine);

    Ok(())
}
