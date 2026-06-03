use crate::{
    app::{App, EditMode}, assembler::{self, Reg}, machine::Machine, runnable::{Loaded, MachineState}
};
use anyhow::Result;
use ratatui::{
    DefaultTerminal, Frame,
    crossterm::{self, event::KeyCode},
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
    loop {
        terminal.draw(render)?;
        if let Some(event) = crossterm::event::read()?.as_key_event() {
            match (app.mode, event.code) {
                (EditMode::Normal, KeyCode::Char(c)) => app.match_normal_command(c),
                KeyCode::Char(c) => match c {
                    'q' => return Ok(()),
                    ''
                },
            };
        }
    }
}

fn render(frame: &mut Frame) {
    frame.render_widget("hello_world", frame.area());
}
