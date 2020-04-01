use crate::interpreter::Instruction;
use crate::lexer::Command;

pub fn parse(commands: Vec<Command>) -> Result<Vec<Instruction>, Error> {
    let mut instructions: Vec<Instruction> = commands
        .iter()
        .map(|cmd| match cmd {
            Command::MoveRight => Instruction::MoveRight(1),
            Command::MoveLeft => Instruction::MoveLeft(1),
            Command::Add => Instruction::Add(1),
            Command::Sub => Instruction::Sub(1),
            Command::JumpIfZero => Instruction::JumpIfZero(0),
            Command::JumpIfNotZero => Instruction::JumpIfNotZero(0),
            Command::Print => Instruction::Print,
            Command::Read => Instruction::Read,
        })
        .collect();

    link_loops(&mut instructions)
}

pub fn link_loops(program: &mut Vec<Instruction>) -> Result<Vec<Instruction>, Error> {
    let mut jump_stack = Vec::new();

    for i in 0..program.len() {
        match program[i] {
            Instruction::JumpIfZero(_) => {
                jump_stack.push(i);
            }
            Instruction::JumpIfNotZero(_) => {
                let jump_index = jump_stack
                    .pop()
                    .ok_or_else(|| Error::Syntax(String::from("Unexpected closing bracket")))?;

                program[i] = Instruction::JumpIfNotZero(jump_index);
                program[jump_index] = Instruction::JumpIfZero(i);
            }
            _ => {}
        }
    }

    if !jump_stack.is_empty() {
        return Err(Error::Syntax(String::from(
            "Opening bracket missing closing bracket",
        )));
    }
    Ok(program.to_vec())
}

#[derive(Debug)]
pub enum Error {
    Syntax(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_parse() {
        let input = vec![
            Command::MoveRight,
            Command::MoveLeft,
            Command::Add,
            Command::Sub,
            Command::JumpIfZero,
            Command::JumpIfNotZero,
            Command::Print,
            Command::Read,
        ];
        let expected = vec![
            Instruction::MoveRight(1),
            Instruction::MoveLeft(1),
            Instruction::Add(1),
            Instruction::Sub(1),
            Instruction::JumpIfZero(5),
            Instruction::JumpIfNotZero(4),
            Instruction::Print,
            Instruction::Read,
        ];
        assert_eq!(parse(input).unwrap(), expected);
    }

    #[test]
    fn test_consecutive_instructions() {
        let input = vec![
            Command::JumpIfZero,
            Command::Add,
            Command::Add,
            Command::JumpIfNotZero,
            Command::Sub,
            Command::Sub,
            Command::Sub,
            Command::Read,
        ];
        let expected = vec![
            Instruction::JumpIfZero(3),
            Instruction::Add(1),
            Instruction::Add(1),
            Instruction::JumpIfNotZero(0),
            Instruction::Sub(1),
            Instruction::Sub(1),
            Instruction::Sub(1),
            Instruction::Read,
        ];
        assert_eq!(parse(input).unwrap(), expected);
    }

    #[test]
    fn test_balanced_brackets() {
        let input = vec![
            Command::JumpIfZero,
            Command::JumpIfZero,
            Command::JumpIfZero,
            Command::JumpIfZero,
            Command::Add,
            Command::JumpIfNotZero,
            Command::JumpIfNotZero,
            Command::JumpIfNotZero,
            Command::JumpIfNotZero,
        ];
        let expected = vec![
            Instruction::JumpIfZero(8),
            Instruction::JumpIfZero(7),
            Instruction::JumpIfZero(6),
            Instruction::JumpIfZero(5),
            Instruction::Add(1),
            Instruction::JumpIfNotZero(3),
            Instruction::JumpIfNotZero(2),
            Instruction::JumpIfNotZero(1),
            Instruction::JumpIfNotZero(0),
        ];
        assert_eq!(parse(input).unwrap(), expected);
    }

    #[test]
    fn test_missing_closing_bracket() {
        let input = vec![Command::JumpIfZero, Command::Add];
        assert!(parse(input).is_err());
    }

    #[test]
    fn test_missing_opening_bracket() {
        let input = vec![Command::Add, Command::JumpIfNotZero];
        assert!(parse(input).is_err());
    }
}
