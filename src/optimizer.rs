use crate::{
    interpreter::Instruction::{
        self, Add, Clear, JumpIfNotZero, JumpIfZero, MoveLeft, MoveRight, Sub,
    },
    parser,
};
use itertools::Itertools;
use std::ops::RangeInclusive;

#[must_use]
pub fn optimize(instructions: Vec<Instruction>) -> Vec<Instruction> {
    let iter = instructions.into_iter();
    let iter = combine_instructions(iter);
    let iter = optimize_clear_loop(iter);
    let mut optimized: Vec<_> = iter.collect();
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
        (Clear, Clear) => Ok(Clear),
        _ => Err((previous, current)),
    })
}

fn optimize_clear_loop<It: Iterator<Item = Instruction>>(
    instructions: It,
) -> impl Iterator<Item = Instruction> {
    fn find_clear_loop(instructions: &[Instruction]) -> Option<RangeInclusive<usize>> {
        let mut run_started = false;
        let mut start = 0;
        for (i, item) in instructions.iter().enumerate() {
            match item {
                JumpIfZero(_a) => {
                    run_started = true;
                    start = i;
                }
                // Unless the argument is odd, the loop is actually infinite and should not
                // be changed into a clear loop. It is probably possible to allow all odd
                // arguments but for now an argument of 1 is the easiest to reason about.
                Sub(1) | Add(1) => { /* noop */ }
                JumpIfNotZero(_a) => {
                    if run_started {
                        // Check to make sure that only a single instruction is between the two brackets
                        if start + 2 == i {
                            return Some(start..=i);
                        }
                        // Reset run
                        run_started = false;
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_combine() {
        use Instruction::*;
        let input = vec![
            Add(1),
            Add(2),
            Add(3),
            Add(4),
            Sub(4),
            Sub(3),
            MoveLeft(2),
            MoveLeft(1),
            MoveRight(3),
            MoveRight(1),
            Clear,
            Clear,
        ];
        let expected = vec![Add(10), Sub(7), MoveLeft(3), MoveRight(4), Clear];
        let optimized: Vec<Instruction> = combine_instructions(input.into_iter()).collect();
        assert_eq!(optimized, expected);
    }

    #[test]
    fn test_no_combinations() {
        use Instruction::*;
        let input = vec![
            Add(1),
            Sub(1),
            Add(1),
            Sub(1),
            Add(1),
            Clear,
            Add(1),
            Clear,
            MoveLeft(1),
            MoveRight(1),
            MoveLeft(1),
            MoveRight(1),
        ];
        let optimized: Vec<Instruction> = combine_instructions(input.clone().into_iter()).collect();
        assert_eq!(optimized, input);
    }

    #[test]
    fn test_non_combinable_instructions() {
        use Instruction::*;
        let input = vec![
            JumpIfZero(1),
            JumpIfNotZero(0),
            JumpIfZero(1),
            JumpIfZero(1),
            JumpIfNotZero(1),
            JumpIfNotZero(1),
            Read,
            Read,
            Print,
            Print,
        ];
        let optimized: Vec<Instruction> = combine_instructions(input.clone().into_iter()).collect();
        assert_eq!(optimized, input);
    }

    #[test]
    fn test_combine_empty_input() {
        let input = vec![];
        let optimized: Vec<Instruction> = combine_instructions(input.into_iter()).collect();
        assert_eq!(optimized, vec![]);
    }

    #[test]
    fn test_clear_loop_empty_input() {
        let input = vec![];
        let optimized: Vec<Instruction> = optimize_clear_loop(input.into_iter()).collect();
        assert_eq!(optimized, vec![]);
    }

    #[test]
    fn test_basic_subtract_clear_loop() {
        use Instruction::*;
        let input = vec![JumpIfZero(2), Sub(1), JumpIfNotZero(0)];
        let optimized: Vec<Instruction> = optimize_clear_loop(input.into_iter()).collect();
        assert_eq!(optimized, vec![Clear]);
    }

    #[test]
    fn test_basic_add_clear_loop() {
        use Instruction::*;
        let input = vec![JumpIfZero(2), Add(1), JumpIfNotZero(0)];
        let optimized: Vec<Instruction> = optimize_clear_loop(input.into_iter()).collect();
        assert_eq!(optimized, vec![Clear]);
    }

    #[test]
    fn test_combined_subtract_clear_loop() {
        use Instruction::*;
        let input = vec![JumpIfZero(2), Sub(5), JumpIfNotZero(0)];
        let optimized: Vec<Instruction> = optimize_clear_loop(input.clone().into_iter()).collect();
        assert_eq!(optimized, input);
    }

    #[test]
    fn test_combined_add_clear_loop() {
        use Instruction::*;
        let input = vec![JumpIfZero(2), Add(5), JumpIfNotZero(0)];
        let optimized: Vec<Instruction> = optimize_clear_loop(input.clone().into_iter()).collect();
        assert_eq!(optimized, input);
    }

    #[test]
    fn test_clear_loop_invalid_input() {
        use Instruction::*;
        let input = vec![JumpIfZero(2), Sub(1), Sub(1), JumpIfNotZero(0)];
        let optimized: Vec<Instruction> = optimize_clear_loop(input.clone().into_iter()).collect();
        assert_eq!(optimized, input);

        let input = vec![JumpIfZero(4), Sub(1), Sub(1), Sub(1), JumpIfNotZero(0)];
        let optimized: Vec<Instruction> = optimize_clear_loop(input.clone().into_iter()).collect();
        assert_eq!(optimized, input);

        let input = vec![
            JumpIfZero(5),
            Sub(1),
            Sub(1),
            Sub(1),
            Sub(1),
            JumpIfNotZero(0),
        ];
        let optimized: Vec<Instruction> = optimize_clear_loop(input.clone().into_iter()).collect();
        assert_eq!(optimized, input);

        let input = vec![
            JumpIfZero(6),
            Sub(1),
            Sub(1),
            Sub(1),
            Sub(1),
            Sub(1),
            JumpIfNotZero(0),
        ];
        let optimized: Vec<Instruction> = optimize_clear_loop(input.clone().into_iter()).collect();
        assert_eq!(optimized, input);

        let input = vec![
            JumpIfZero(4),
            JumpIfZero(3),
            MoveRight(1),
            JumpIfNotZero(1),
            JumpIfNotZero(0),
        ];
        let optimized: Vec<Instruction> = optimize_clear_loop(input.clone().into_iter()).collect();
        assert_eq!(optimized, input);

        let input = vec![JumpIfZero(2), Sub(1), Add(1), JumpIfNotZero(0)];
        let optimized: Vec<Instruction> = optimize_clear_loop(input.clone().into_iter()).collect();
        assert_eq!(optimized, input);

        let input = vec![JumpIfZero(1), JumpIfNotZero(0)];
        let optimized: Vec<Instruction> = optimize_clear_loop(input.clone().into_iter()).collect();
        assert_eq!(optimized, input);
    }
}
