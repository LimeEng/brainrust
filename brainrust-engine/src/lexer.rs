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

pub fn lex(input: &String) -> Vec<Command> {
    input
        .chars()
        .fold(Vec::with_capacity(input.len()), |mut acc, chr| {
            if let Some(instruction) = lex_char(chr) {
                acc.push(instruction);
            }
            acc
        })
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
