use crate::program::{Instruction, Program};
use std::io;

#[derive(Debug)]
pub enum Error {
    Io(io::Error),
    PointerOverflow,
    PointerUnderflow,
}

impl From<io::Error> for Error {
    fn from(error: io::Error) -> Self {
        Error::Io(error)
    }
}

pub struct Tape<'a> {
    input: &'a mut dyn io::Read,
    output: &'a mut dyn io::Write,
    memory: Vec<u8>,
    pointer: usize,
}

impl<'a> Tape<'a> {
    #[must_use]
    pub fn new(
        input: &'a mut dyn io::Read,
        output: &'a mut dyn io::Write,
        memory_size: usize,
    ) -> Self {
        Self {
            input,
            output,
            memory: vec![0; memory_size],
            pointer: 0,
        }
    }

    pub fn execute(&mut self, program: &Program) -> Result<(), Error> {
        self.execute_instructions(program.instructions())
    }

    fn execute_instructions(&mut self, instructions: &[Instruction]) -> Result<(), Error> {
        let mut next_instruction = 0;

        while next_instruction < instructions.len() {
            match &instructions[next_instruction] {
                Instruction::MoveRight(value) => self.move_pointer_right(*value)?,
                Instruction::MoveLeft(value) => self.move_pointer_left(*value)?,
                Instruction::Add(value) => self.increment_current_cell(*value as u8),
                Instruction::Sub(value) => self.decrement_current_cell(*value as u8),
                Instruction::Loop { body } => {
                    while self.read_current_cell() != 0 {
                        self.execute_instructions(body)?;
                    }
                }
                Instruction::Print => self.print()?,
                Instruction::Read => self.read()?,
                Instruction::Set(value) => self.write_current_cell(*value as u8),
            }
            next_instruction += 1;
        }
        Ok(())
    }

    fn read_current_cell(&self) -> u8 {
        self.memory[self.pointer]
    }

    fn write_current_cell(&mut self, value: u8) {
        self.memory[self.pointer] = value;
    }

    fn increment_current_cell(&mut self, value: u8) {
        let value = self.read_current_cell().wrapping_add(value);
        self.write_current_cell(value);
    }

    fn decrement_current_cell(&mut self, value: u8) {
        let value = self.read_current_cell().wrapping_sub(value);
        self.write_current_cell(value);
    }

    fn move_pointer_right(&mut self, steps: usize) -> Result<(), Error> {
        if self.pointer + steps >= self.memory.len() {
            return Err(Error::PointerOverflow);
        }
        self.pointer += steps;
        Ok(())
    }

    fn move_pointer_left(&mut self, steps: usize) -> Result<(), Error> {
        if self.pointer < steps {
            return Err(Error::PointerUnderflow);
        }
        self.pointer -= steps;
        Ok(())
    }

    fn print(&mut self) -> Result<(), Error> {
        self.output.write_all(&[self.read_current_cell()])?;
        Ok(())
    }

    fn read(&mut self) -> Result<(), Error> {
        let mut buffer = [0; 1];
        let bytes = self.input.read(&mut buffer)?;
        let value = if bytes > 0 { buffer[0] } else { 0 };
        self.write_current_cell(value);
        Ok(())
    }
}
