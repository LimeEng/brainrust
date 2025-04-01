use super::{Error, tape::Tape};
use crate::program::{Instruction, Program};
use std::{collections::HashMap, io};

#[derive(Clone, Debug, Default)]
pub struct Analytics {
    pub frequency: HashMap<Instruction, u64>,
    pub loop_patterns: HashMap<Vec<Instruction>, u64>,
    pub highest_memory_access: usize,
}

pub fn profile(
    program: &Program,
    input: &mut dyn io::Read,
    output: &mut dyn io::Write,
    memory_size: usize,
) -> Result<Analytics, Error> {
    let mut tape = Tape::new(input, output, memory_size);
    let mut analytics = Analytics::default();

    execute_instructions(&mut tape, program.instructions(), &mut analytics)?;
    Ok(analytics)
}

fn contains_loop(instructions: &[Instruction]) -> bool {
    instructions
        .iter()
        .any(|instruction| matches!(instruction, Instruction::Loop { .. }))
}

fn execute_instructions(
    tape: &mut Tape,
    instructions: &[Instruction],
    analytics: &mut Analytics,
) -> Result<(), Error> {
    for instruction in instructions {
        execute_instruction(tape, instruction, analytics)?;

        if let Instruction::Loop { body } = instruction {
            if !contains_loop(body) {
                *analytics.loop_patterns.entry(body.clone()).or_insert(0) += 1;
            }
        } else {
            *analytics.frequency.entry(instruction.clone()).or_insert(0) += 1;
        }
        analytics.highest_memory_access = analytics.highest_memory_access.max(tape.pointer());
    }
    Ok(())
}

fn execute_instruction(
    tape: &mut Tape,
    instruction: &Instruction,
    analytics: &mut Analytics,
) -> Result<(), Error> {
    match instruction {
        Instruction::MoveRight(value) => tape.move_pointer_right(*value)?,
        Instruction::MoveLeft(value) => tape.move_pointer_left(*value)?,
        Instruction::Add(value) => tape.increment_current_cell(*value as u8),
        Instruction::Sub(value) => tape.decrement_current_cell(*value as u8),
        Instruction::Loop { body } => {
            while tape.read_current_cell() != 0 {
                execute_instructions(tape, body, analytics)?;
            }
        }
        Instruction::Print => tape.print()?,
        Instruction::Read => tape.read()?,
        Instruction::Set(value) => tape.write_current_cell(*value as u8),
    }
    Ok(())
}
