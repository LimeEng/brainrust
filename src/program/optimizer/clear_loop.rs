use crate::program::Instruction;

pub fn optimize(instructions: &mut Vec<Instruction>) {
    for instruction in instructions {
        if let Instruction::Loop { body } = instruction {
            if matches!(body.as_slice(), [Instruction::Add(1) | Instruction::Sub(1)]) {
                *instruction = Instruction::Set(0);
            } else {
                optimize(body);
            }
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
    fn test_clear_loop_empty_input() {
        assert_optimizes_to(vec![], &[]);
    }

    #[test]
    fn test_basic_subtract_clear_loop() {
        let input = vec![Instruction::Loop {
            body: vec![Instruction::Sub(1)],
        }];
        assert_optimizes_to(input, &[Instruction::Set(0)]);
    }

    #[test]
    fn test_basic_add_clear_loop() {
        let input = vec![Instruction::Loop {
            body: vec![Instruction::Add(1)],
        }];
        assert_optimizes_to(input, &[Instruction::Set(0)]);
    }

    #[test]
    fn test_combined_subtract_clear_loop() {
        let input = vec![Instruction::Loop {
            body: vec![Instruction::Sub(5)],
        }];
        assert_optimizes_to(input.clone(), &input.clone());
    }

    #[test]
    fn test_combined_add_clear_loop() {
        let input = vec![Instruction::Loop {
            body: vec![Instruction::Add(5)],
        }];
        assert_optimizes_to(input.clone(), &input.clone());
    }

    #[test]
    fn test_double_subtract_clear_loop() {
        let input = vec![Instruction::Loop {
            body: vec![Instruction::Sub(1), Instruction::Sub(1)],
        }];
        assert_optimizes_to(input.clone(), &input.clone());
    }

    #[test]
    fn test_double_add_clear_loop() {
        let input = vec![Instruction::Loop {
            body: vec![Instruction::Add(1), Instruction::Add(1)],
        }];
        assert_optimizes_to(input.clone(), &input.clone());
    }

    #[test]
    fn test_nested_subtract_clear_loop() {
        let input = vec![Instruction::Loop {
            body: vec![Instruction::Loop {
                body: vec![Instruction::Sub(1)],
            }],
        }];
        assert_optimizes_to(
            input,
            &[Instruction::Loop {
                body: vec![Instruction::Set(0)],
            }],
        );
    }

    #[test]
    fn test_nested_add_clear_loop() {
        let input = vec![Instruction::Loop {
            body: vec![Instruction::Loop {
                body: vec![Instruction::Add(1)],
            }],
        }];
        assert_optimizes_to(
            input,
            &[Instruction::Loop {
                body: vec![Instruction::Set(0)],
            }],
        );
    }
}
