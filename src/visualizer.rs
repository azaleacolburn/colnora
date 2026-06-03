use crate::{
    app::{App, EditMode},
    assembler::{self},
    instruction::Instruction,
    machine::Machine,
    reg::Reg,
    runnable::{Loaded, MachineState, Unloaded},
};
use anyhow::Result;
use ratatui::{
    DefaultTerminal, Frame,
    crossterm::{self, event::KeyCode},
    layout::{Constraint, Direction, Layout},
    style::Stylize,
    widgets::{Block, Paragraph},
};
use std::fs::read_to_string;

pub fn read_instructions_file<S: MachineState>(machine: Machine<S>) -> Result<Machine<Loaded>> {
    let instructions_str = read_to_string("tests/hello_name.s")?;
    let assembly_file = assembler::parse_file(&instructions_str);
    println!("{:?}", assembly_file.instructions);

    // This is ridiculous, of course instead they should do:
    // let mut machine = Machine::<Loaded>::new(assembly_file);
    Ok(machine.load_all(assembly_file))
}

pub fn app(terminal: &mut DefaultTerminal) -> anyhow::Result<()> {
    let mut app = App::new();
    let mut machine = Machine::<Unloaded>::new();
    loop {
        terminal.draw(|frame: &mut Frame| render(frame, machine))?;
        if let Some(event) = crossterm::event::read()?.as_key_event() {
            let stop = match (app.mode, event.code) {
                (EditMode::Normal, KeyCode::Char(c)) => app.match_normal_command(c),
                _ => false,
            };

            if stop {
                break;
            }
        }
    }

    Ok(())
}

fn instruction_block() {
    let instruction_block = Block::bordered().title("Instructions");
}

fn render(frame: &mut Frame) {
    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![
            Constraint::Percentage(50),
            Constraint::Percentage(25),
            Constraint::Percentage(25),
        ])
        .split(frame.area());

    let window = Block::bordered().title("Colnora Visual Interface");
    let title = Paragraph::new("").centered().yellow().block(window);

    let register_block = Block::bordered().title("Registers");
    let stack_block = Block::bordered().title("Stack");

    frame.render_widget(instruction_block, frame.area());
}

fn render_instructions(instructions: &[Instruction]) -> Vec<String> {
    instructions
        .iter()
        .map(|inst| {
            let mut str = inst.to_string();
            str.push('\n');
            str
        })
        .collect()
}
