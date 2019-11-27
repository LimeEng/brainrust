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

    link_loops(&mut instructions)?;

    Ok(instructions)
}

fn link_loops(program: &mut [Instruction]) -> Result<(), Error> {
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
    Ok(())
}

#[derive(Debug)]
pub enum Error {
    Syntax(String),
}
