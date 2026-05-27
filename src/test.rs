use std::fs::{read_dir, read_to_string};

use crate::{assembler, machine::Machine};

#[test]
fn test_all() -> anyhow::Result<()> {
    let tests = read_dir("tests")?;

    for test in tests {
        if let Ok(dir) = test {
            let name = dir.file_name();
            let name = name.to_str().unwrap().split('.').next().unwrap();

            let instructions = read_to_string(dir.path())?;
            let instructions = assembler::parse_instructions(&instructions);

            let mut machine = Machine::new();
            match machine.run(&instructions) {
                Ok(_) => println!("`{}` Test Succeeded", name),
                Err(_) => println!("`{}` Test Failed", name),
            }
        }
    }

    Ok(())
}
