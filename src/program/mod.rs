mod lexer;
mod optimizer;
mod parser;

use std::collections::HashMap;

pub use parser::Error;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Instruction {
    MoveRight(usize),
    MoveLeft(usize),
    Add(usize),
    Sub(usize),
    Loop { body: Vec<Instruction> },
    Print,
    Read,
    Set(usize),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Program {
    instructions: Vec<Instruction>,
}

impl From<Vec<Instruction>> for Program {
    fn from(instructions: Vec<Instruction>) -> Self {
        Self { instructions }
    }
}

impl Program {
    pub fn parse(input: &str) -> Result<Self, Error> {
        let instructions = parser::parse(input)?;
        Ok(Self { instructions })
    }

    #[must_use]
    pub fn optimized(&self) -> Self {
        let mut instructions = self.instructions.clone();
        optimizer::optimize(&mut instructions);
        Self::from(instructions)
    }

    #[must_use]
    pub fn instructions(&self) -> &[Instruction] {
        &self.instructions
    }

    /// Computes Shannon entropy (in bits) for a sequence of instructions.
    /// [Link](https://en.wikipedia.org/wiki/Entropy_(information_theory))
    #[must_use]
    pub fn entropy_in_bits(&self) -> f64 {
        // TODO: Descend into the loop bodies
        let total = self.instructions.len() as f64;
        self.instructions
            .iter()
            .fold(HashMap::new(), |mut acc, instr| {
                *acc.entry(instr).or_insert(0) += 1;
                acc
            })
            .values()
            .map(|&count| {
                let p = f64::from(count) / total;
                -p * p.log2()
            })
            .sum()
    }
}
