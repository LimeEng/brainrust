use super::Instruction;
use crate::program::util;

pub fn optimize(instructions: &mut Vec<Instruction>) {
    combine_instructions(instructions);
    optimize_clear_loop(instructions);
}

fn combine_instructions(instructions: &mut Vec<Instruction>) {
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

fn optimize_clear_loop(instructions: &mut Vec<Instruction>) {
    for instruction in instructions {
        if let Instruction::Loop { body } = instruction {
            if matches!(body.as_slice(), [Instruction::Add(1) | Instruction::Sub(1)]) {
                *instruction = Instruction::Set(0);
            } else {
                optimize_clear_loop(body);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_combine() {
        let mut input = vec![
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
        combine_instructions(&mut input);
        assert_eq!(input, expected);
    }

    #[test]
    fn test_no_combinations() {
        let mut input = vec![
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
        let expected = input.clone();
        combine_instructions(&mut input);
        assert_eq!(input, expected);
    }

    #[test]
    fn test_non_combinable_instructions() {
        let mut input = vec![
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
        let expected = input.clone();
        combine_instructions(&mut input);
        assert_eq!(input, expected);
    }

    #[test]
    fn test_combine_empty_input() {
        let mut input = vec![];
        combine_instructions(&mut input);
        assert_eq!(input, vec![]);
    }

    #[test]
    fn test_combine_multiple_nested_loops() {
        let mut input = vec![Instruction::Loop {
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
        combine_instructions(&mut input);
        assert_eq!(input, expected);
    }

    #[test]
    fn test_clear_loop_empty_input() {
        let mut input = vec![];
        optimize_clear_loop(&mut input);
        assert_eq!(input, vec![]);
    }

    #[test]
    fn test_basic_subtract_clear_loop() {
        let mut input = vec![Instruction::Loop {
            body: vec![Instruction::Sub(1)],
        }];
        optimize_clear_loop(&mut input);
        assert_eq!(input, &[Instruction::Set(0)]);
    }

    #[test]
    fn test_basic_add_clear_loop() {
        let mut input = vec![Instruction::Loop {
            body: vec![Instruction::Add(1)],
        }];
        optimize_clear_loop(&mut input);
        assert_eq!(input, &[Instruction::Set(0)]);
    }

    #[test]
    fn test_combined_subtract_clear_loop() {
        let mut input = vec![Instruction::Loop {
            body: vec![Instruction::Sub(5)],
        }];
        let expected = input.clone();
        optimize_clear_loop(&mut input);
        assert_eq!(input, expected);
    }

    #[test]
    fn test_combined_add_clear_loop() {
        let mut input = vec![Instruction::Loop {
            body: vec![Instruction::Add(5)],
        }];
        let expected = input.clone();
        optimize_clear_loop(&mut input);
        assert_eq!(input, expected);
    }

    #[test]
    fn test_double_subtract_clear_loop() {
        let mut input = vec![Instruction::Loop {
            body: vec![Instruction::Sub(1), Instruction::Sub(1)],
        }];
        let expected = input.clone();
        optimize_clear_loop(&mut input);
        assert_eq!(input, expected);
    }

    #[test]
    fn test_double_add_clear_loop() {
        let mut input = vec![Instruction::Loop {
            body: vec![Instruction::Add(1), Instruction::Add(1)],
        }];
        let expected = input.clone();
        optimize_clear_loop(&mut input);
        assert_eq!(input, expected);
    }

    #[test]
    fn test_nested_subtract_clear_loop() {
        let mut input = vec![Instruction::Loop {
            body: vec![Instruction::Loop {
                body: vec![Instruction::Sub(1)],
            }],
        }];
        let expected = &[Instruction::Loop {
            body: vec![Instruction::Set(0)],
        }];
        optimize_clear_loop(&mut input);
        assert_eq!(input, expected);
    }

    #[test]
    fn test_nested_add_clear_loop() {
        let mut input = vec![Instruction::Loop {
            body: vec![Instruction::Loop {
                body: vec![Instruction::Add(1)],
            }],
        }];
        let expected = &[Instruction::Loop {
            body: vec![Instruction::Set(0)],
        }];
        optimize_clear_loop(&mut input);
        assert_eq!(input, expected);
    }
}
