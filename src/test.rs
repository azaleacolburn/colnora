use std::fs::{read_dir, read_to_string};

use anyhow::bail;

use crate::{assembler, machine::Machine, runnable::Unloaded};

#[test]
fn test_all() -> anyhow::Result<()> {
    let tests = read_dir("tests")?;
    let mut failed: Vec<String> = vec![];

    for test in tests {
        if let Ok(dir) = test {
            let name = dir.file_name();
            let name = name.to_str().unwrap().split('.').next().unwrap();

            let file_text = read_to_string(dir.path())?;
            let assembly_file = assembler::parse_file(&file_text);

            let machine = Machine::<Unloaded>::new();
            let mut machine = machine.load_all(assembly_file);
            match machine.run() {
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
