use std::fs::read_to_string;

// use crate::visualizer::app;
use anyhow::Result;

use crate::{machine::Machine, runnable::Unloaded};

pub mod app;
pub mod assembler;
pub mod instruction;
pub mod machine;
pub mod reg;
// pub mod render;
pub mod runnable;
#[cfg(test)]
mod test;
pub mod value;
// pub mod visualizer;

fn main() -> Result<()> {
    let instructions_str = read_to_string("tests/atoi.s")?;
    let assembly_file = assembler::parse_file(&instructions_str);
    println!("{:?}", assembly_file.instructions);

    let machine = Machine::<Unloaded>::new();
    let mut machine = machine.load_all(assembly_file);

    machine.run()?;

    println!("{:?}", machine);

    Ok(())
}

// fn main() -> Result<()> {
//     ratatui::run(app)?;
//
//     Ok(())
// }
