use crate::assembler::Reg;

pub enum EditMode {
    Normal,
    // Entering a command
    Command,
    // Editing Instructions
    Instruction,
    // Editing the Stack
    Stack,
    // Editing a Register
    Registers,
}

pub struct App {
    command: String,
    command_history: Vec<String>,
    // Index within the command history (for searching backwards)
    command_history_idx: usize,
    // Index within the command text box.
    command_edit_idx: usize,

    // Positions within different segments of the program that are being edited
    // These could be stored within the `EditMode` enum, but then they wouldn't be saved when you
    // move to some other part of the program
    instruction_position: usize,
    stack_position: usize,
    register_being_edited: Reg,

    mode: EditMode,
}

impl App {
    pub fn new() -> Self {
        Self {
            command: String::new(),
            command_history: Vec::new(),
            // Index within the command history (for searching backwards)
            command_history_idx: 0,
            // Index within the command text box.
            command_edit_idx: 0,

            // Positions within different segments of the program that are being edited
            // These could be stored within the `EditMode` enum, but then they wouldn't be saved when you
            // move to some other part of the program
            instruction_position: 0,
            stack_position: 0,
            register_being_edited: Reg::A,

            mode: EditMode::Normal,
        }
    }

    pub fn match_normal_command(&mut self, c: char) -> bool {
        match c {
            'q' => {
                return true;
            }
            'c' => self.mode = EditMode::Command,
            'r' => self.mode = EditMode::Registers,
            's' => self.mode = EditMode::Stack,
            _ => {}
        };

        false
    }
}
