use super::{
    Instruction,
    lexer::{Command, lex},
};

#[derive(Debug)]
pub enum Error {
    Syntax(String),
}

pub fn parse(text: &str) -> Result<Vec<Instruction>, Error> {
    let commands = lex(text);
    // Instructions encountered in the current block
    let mut instructions = vec![];
    let mut loop_stack = vec![];

    for cmd in commands {
        match cmd {
            Command::MoveRight => instructions.push(Instruction::MoveRight(1)),
            Command::MoveLeft => instructions.push(Instruction::MoveLeft(1)),
            Command::Add => instructions.push(Instruction::Add(1)),
            Command::Sub => instructions.push(Instruction::Sub(1)),
            Command::Print => instructions.push(Instruction::Print),
            Command::Read => instructions.push(Instruction::Read),
            Command::JumpIfZero => {
                loop_stack.push(instructions);
                instructions = vec![];
            }
            Command::JumpIfNotZero => match loop_stack.pop() {
                Some(mut parent) => {
                    parent.push(Instruction::Loop { body: instructions });
                    instructions = parent;
                }
                None => {
                    return Err(Error::Syntax("Missing opening bracket".into()));
                }
            },
        }
    }

    if !loop_stack.is_empty() {
        return Err(Error::Syntax("Missing closing bracket".into()));
    }

    Ok(instructions)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_parse() {
        let input = "><+-[].,";
        let expected = vec![
            Instruction::MoveRight(1),
            Instruction::MoveLeft(1),
            Instruction::Add(1),
            Instruction::Sub(1),
            Instruction::Loop { body: vec![] },
            Instruction::Print,
            Instruction::Read,
        ];
        assert_eq!(parse(input).unwrap(), expected);
    }

    #[test]
    fn test_consecutive_instructions() {
        let input = "[++]---,";
        let expected = vec![
            Instruction::Loop {
                body: vec![Instruction::Add(1), Instruction::Add(1)],
            },
            Instruction::Sub(1),
            Instruction::Sub(1),
            Instruction::Sub(1),
            Instruction::Read,
        ];
        assert_eq!(parse(input).unwrap(), expected);
    }

    #[test]
    fn test_balanced_brackets() {
        let input = "[[[[+]]]]";
        let expected = vec![Instruction::Loop {
            body: vec![Instruction::Loop {
                body: vec![Instruction::Loop {
                    body: vec![Instruction::Loop {
                        body: vec![Instruction::Add(1)],
                    }],
                }],
            }],
        }];
        assert_eq!(parse(input).unwrap(), expected);
    }

    #[test]
    fn test_missing_closing_bracket() {
        assert!(parse("[+").is_err());
    }

    #[test]
    fn test_missing_opening_bracket() {
        assert!(parse("+]").is_err());
    }
}
