use crate::program::{Instruction, optimizer::util};

pub fn optimize(instructions: &mut Vec<Instruction>) {
    use Instruction as Instr;

    // Coalesce the current block
    util::coalesce(instructions, |current, next| match (current, next) {
        (Instr::MoveRight(a), Instr::MoveRight(b)) => Some(Instr::MoveRight(a + b)),
        (Instr::MoveLeft(a), Instr::MoveLeft(b)) => Some(Instr::MoveLeft(a + b)),
        (Instr::Add(a), Instr::Add(b)) => Some(Instr::Add(a + b)),
        (Instr::Sub(a), Instr::Sub(b)) => Some(Instr::Sub(a + b)),
        (Instr::Set(_), Instr::Set(b)) => Some(Instr::Set(*b)),
        _ => None,
    });

    // Recursively handle loops
    for instruction in instructions.iter_mut() {
        if let Instr::Loop { body } = instruction {
            optimize(body);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_optimizes_to(input: Vec<Instruction>, expected: &[Instruction]) {
        let mut input = input;
        optimize(&mut input);
        assert_eq!(input, expected);
    }

    #[test]
    fn test_basic_combine() {
        let input = vec![
            Instruction::Add(1),
            Instruction::Add(2),
            Instruction::Add(3),
            Instruction::Add(4),
            Instruction::Sub(4),
            Instruction::Sub(3),
            Instruction::MoveLeft(2),
            Instruction::MoveLeft(1),
            Instruction::MoveRight(3),
            Instruction::MoveRight(1),
            Instruction::Set(0),
            Instruction::Set(0),
        ];
        let expected = vec![
            Instruction::Add(10),
            Instruction::Sub(7),
            Instruction::MoveLeft(3),
            Instruction::MoveRight(4),
            Instruction::Set(0),
        ];
        assert_optimizes_to(input, &expected);
    }

    #[test]
    fn test_no_combinations() {
        let input = vec![
            Instruction::Add(1),
            Instruction::Sub(1),
            Instruction::Add(1),
            Instruction::Sub(1),
            Instruction::Add(1),
            Instruction::Set(0),
            Instruction::Add(1),
            Instruction::Set(0),
            Instruction::MoveLeft(1),
            Instruction::MoveRight(1),
            Instruction::MoveLeft(1),
            Instruction::MoveRight(1),
        ];
        assert_optimizes_to(input.clone(), &input.clone());
    }

    #[test]
    fn test_non_combinable_instructions() {
        let input = vec![
            Instruction::Loop { body: vec![] },
            Instruction::Loop {
                body: vec![Instruction::Read, Instruction::Read],
            },
            Instruction::Loop { body: vec![] },
            Instruction::Read,
            Instruction::Read,
            Instruction::Print,
            Instruction::Print,
        ];
        assert_optimizes_to(input.clone(), &input.clone());
    }

    #[test]
    fn test_combine_empty_input() {
        assert_optimizes_to(vec![], &[]);
    }

    #[test]
    fn test_combine_multiple_nested_loops() {
        let input = vec![Instruction::Loop {
            body: vec![
                Instruction::Add(1),
                Instruction::Add(2),
                Instruction::Loop {
                    body: vec![
                        Instruction::MoveRight(1),
                        Instruction::MoveRight(2),
                        Instruction::Loop {
                            body: vec![Instruction::Sub(1), Instruction::Sub(2)],
                        },
                    ],
                },
            ],
        }];
        let expected = vec![Instruction::Loop {
            body: vec![
                Instruction::Add(3),
                Instruction::Loop {
                    body: vec![
                        Instruction::MoveRight(3),
                        Instruction::Loop {
                            body: vec![Instruction::Sub(3)],
                        },
                    ],
                },
            ],
        }];
        assert_optimizes_to(input, &expected);
    }
}
