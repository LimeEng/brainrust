use crate::interpreter::Instruction;
use crate::interpreter::Instruction::*;
use crate::parser;

use itertools::Itertools;

pub fn optimize(instructions: Vec<Instruction>) -> Vec<Instruction> {
    let mut optimized: Vec<Instruction> = combine_instructions(instructions.into_iter()).collect();
    parser::link_loops(&mut optimized).unwrap()
}

fn combine_instructions<It: Iterator<Item = Instruction>>(
    instructions: It,
) -> impl Iterator<Item = Instruction> {
    instructions.coalesce(|previous, current| match (previous, current) {
        (MoveRight(a), MoveRight(b)) => Ok(MoveRight(a + b)),
        (MoveLeft(a), MoveLeft(b)) => Ok(MoveLeft(a + b)),
        (Add(a), Add(b)) => Ok(Add(a + b)),
        (Sub(a), Sub(b)) => Ok(Sub(a + b)),
        _ => Err((previous, current)),
    })
}
