use anyhow::Result;
use std::fs::read_to_string;

use crate::{
    machine::Machine,
    runnable::{Runnable, Unloaded},
    visualizer::app,
};

pub mod app;
pub mod assembler;
pub mod machine;
pub mod runnable;
#[cfg(test)]
mod test;
pub mod visualizer;

// fn main() -> Result<()> {
//     let instructions_str = read_to_string("tests/hello_name.s")?;
//     let assembly_file = assembler::parse_file(&instructions_str);
//     println!("{:?}", assembly_file.instructions);
//
//     let mut machine = Machine::new();
//     machine.load(&assembly_file.data);
//
//     machine.run(&assembly_file.instructions)?;
//
//     println!("{:?}", machine);
//
//     Ok(())
// }
fn main() -> Result<()> {
    ratatui::run(app)?;

    Ok(())
}
