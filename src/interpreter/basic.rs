use super::{Error, tape::Tape};
use crate::program::{Instruction, Program};
use std::io;

pub fn execute(
    program: &Program,
    input: &mut dyn io::Read,
    output: &mut dyn io::Write,
    memory_size: usize,
) -> Result<(), Error> {
    let mut tape = Tape::new(input, output, memory_size);
    execute_instructions(&mut tape, program.instructions())?;
    Ok(())
}

fn execute_instructions(tape: &mut Tape, instructions: &[Instruction]) -> Result<(), Error> {
    for instruction in instructions {
        execute_instruction(tape, instruction)?;
    }
    Ok(())
}

fn execute_instruction(tape: &mut Tape, instruction: &Instruction) -> Result<(), Error> {
    match instruction {
        Instruction::MoveRight(value) => tape.move_pointer_right(*value)?,
        Instruction::MoveLeft(value) => tape.move_pointer_left(*value)?,
        Instruction::Add(value) => tape.increment_current_cell(*value as u8),
        Instruction::Sub(value) => tape.decrement_current_cell(*value as u8),
        Instruction::Loop { body } => {
            while tape.read_current_cell() != 0 {
                execute_instructions(tape, body)?;
            }
        }
        Instruction::Print => tape.print()?,
        Instruction::Read => tape.read()?,
        Instruction::Set(value) => tape.write_current_cell(*value as u8),
    }
    Ok(())
}
