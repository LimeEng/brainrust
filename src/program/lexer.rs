#[derive(Debug, PartialEq)]
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

pub fn lex(text: &str) -> Vec<Command> {
    text.chars().filter_map(lex_char).collect()
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_lex() {
        let input = "><+-[].,";
        let expected = vec![
            Command::MoveRight,
            Command::MoveLeft,
            Command::Add,
            Command::Sub,
            Command::JumpIfZero,
            Command::JumpIfNotZero,
            Command::Print,
            Command::Read,
        ];
        assert_eq!(lex(input), expected);
    }

    #[test]
    fn test_lex_with_unicode() {
        let input = "🦀🦀🦀>🐚🐚<🌴🌴+🌊-🌊[🌊]🌊.🌊,🌊";
        let expected = vec![
            Command::MoveRight,
            Command::MoveLeft,
            Command::Add,
            Command::Sub,
            Command::JumpIfZero,
            Command::JumpIfNotZero,
            Command::Print,
            Command::Read,
        ];
        assert_eq!(lex(input), expected);
    }

    #[test]
    fn test_with_empty_input() {
        assert_eq!(lex(""), []);
    }

    #[test]
    fn test_with_invalid_input() {
        let input = "What a beautiful 🦀🦀🦀🐚🐚🌴🌴🌊🌊🌊🌊🌊🌊 ocean landscape!";
        assert_eq!(lex(input), []);
    }
}
