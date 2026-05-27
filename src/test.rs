use std::fs::{read_dir, read_to_string};

use anyhow::bail;

use crate::{assembler, machine::Machine};

#[test]
fn test_all() -> anyhow::Result<()> {
    let tests = read_dir("tests")?;
    let mut failed: Vec<String> = vec![];

    for test in tests {
        if let Ok(dir) = test {
            let name = dir.file_name();
            let name = name.to_str().unwrap().split('.').next().unwrap();

            let instructions = read_to_string(dir.path())?;
            let instructions = assembler::parse_instructions(&instructions);

            let mut machine = Machine::new();
            match machine.run(&instructions) {
                Ok(_) => println!("`{}` Test Succeeded", name),
                Err(_) => {
                    failed.push(name.to_string());
                    println!("`{}` Test Failed", name)
                }
            }
        }
    }

    if failed.is_empty() {
        Ok(())
    } else {
        bail!("The following tests failed: {:?}", failed);
    }
}
