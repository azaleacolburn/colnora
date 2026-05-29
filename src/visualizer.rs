use ratatui::{DefaultTerminal, Frame, crossterm};

pub fn app(terminal: &mut DefaultTerminal) -> anyhow::Result<()> {
    loop {
        terminal.draw(render)?;
        if crossterm::event::read()?.is_key_press() {
            break Ok(());
        }
    }
}

fn render(frame: &mut Frame) {
    frame.render_widget("hello_world", frame.area());
}
