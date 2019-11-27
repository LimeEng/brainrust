use crate::instruction::Instruction;

pub fn parse(input: &String) -> Result<Vec<Instruction>, Error> {
    // TODO: Initialize Vec with some size, maybe input.len()?
    let mut program = input.chars().fold(Vec::new(), |mut acc, chr| {
        if let Some(instruction) = lex_char(chr) {
            acc.push(instruction);
        }
        acc
    });

    link_loops(&mut program)?;

    Ok(program)
}

fn lex_char(chr: char) -> Option<Instruction> {
    match chr {
        '>' => Some(Instruction::MoveRight(1)),
        '<' => Some(Instruction::MoveLeft(1)),
        '+' => Some(Instruction::Add(1)),
        '-' => Some(Instruction::Sub(1)),
        '[' => Some(Instruction::JumpIfZero(0)),
        ']' => Some(Instruction::JumpIfNotZero(1)),
        '.' => Some(Instruction::Print),
        ',' => Some(Instruction::Read),
        _ => None,
    }
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
