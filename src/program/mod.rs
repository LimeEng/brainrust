mod lexer;
mod optimizer;
mod parser;

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
}
