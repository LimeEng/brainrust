use std::ops::Range;

use itertools::Itertools;

use crate::interpreter::Instruction;
use crate::interpreter::Instruction::*;
use crate::parser;

pub fn optimize(instructions: Vec<Instruction>) -> Vec<Instruction> {
    let iter = instructions.into_iter();
    let iter = combine_instructions(iter);
    let iter = optimize_clear_loop(iter);
    let iter = remove_mutation_before_input(iter);
    let mut optimized = iter.collect();
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

fn optimize_clear_loop<It: Iterator<Item = Instruction>>(
    instructions: It,
) -> impl Iterator<Item = Instruction> {
    fn find_clear_loop(instructions: &Vec<Instruction>) -> Option<Range<usize>> {
        let mut run_started = false;
        let mut start = 0;
        for (i, item) in instructions.iter().enumerate() {
            match item {
                JumpIfZero(_a) => {
                    run_started = true;
                    start = i;
                }
                Sub(_a) => { /* noop */ }
                JumpIfNotZero(_a) => {
                    if run_started {
                        return Some(start..(i + 1));
                    }
                }
                _ => {
                    run_started = false;
                }
            };
        }
        None
    }

    let mut instructions = instructions.collect::<Vec<Instruction>>();

    while let Some(range) = find_clear_loop(&instructions) {
        instructions.splice(range, vec![Clear].into_iter());
    }

    instructions.into_iter()
}

fn remove_mutation_before_input<It: Iterator<Item = Instruction>>(
    instructions: It,
) -> impl Iterator<Item = Instruction> {
    instructions.coalesce(|previous, current| match (previous, current) {
        (Add(_), Read) => Ok(Read),
        (Sub(_), Read) => Ok(Read),
        (Clear, Read) => Ok(Read),
        _ => Err((previous, current)),
    })
}
