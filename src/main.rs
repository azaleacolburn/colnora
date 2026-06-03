use crate::visualizer::app;
use anyhow::Result;

pub mod app;
pub mod assembler;
pub mod instruction;
pub mod machine;
pub mod reg;
pub mod runnable;
#[cfg(test)]
mod test;
pub mod value;
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
