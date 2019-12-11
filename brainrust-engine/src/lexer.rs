pub enum Command {
    MoveRight,
    MoveLeft,
    Add,
    Sub,
    JumpIfZero,
    JumpIfNotZero,
    Print,
    Read,
}

pub fn lex(input: &str) -> Vec<Command> {
    input.chars().filter_map(lex_char).collect()
}

fn lex_char(chr: char) -> Option<Command> {
    match chr {
        '>' => Some(Command::MoveRight),
        '<' => Some(Command::MoveLeft),
        '+' => Some(Command::Add),
        '-' => Some(Command::Sub),
        '[' => Some(Command::JumpIfZero),
        ']' => Some(Command::JumpIfNotZero),
        '.' => Some(Command::Print),
        ',' => Some(Command::Read),
        _ => None,
    }
}
