use std::io;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Instruction {
    MoveRight(usize),
    MoveLeft(usize),
    Add(usize),
    Sub(usize),
    JumpIfZero(usize),
    JumpIfNotZero(usize),
    Print,
    Read,
    Clear,
}

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

pub struct Interpreter {
    memory: Vec<u8>,
    pointer: usize,
}

impl Interpreter {
    #[must_use]
    pub fn new(memory_size: usize) -> Self {
        Interpreter {
            memory: vec![0; memory_size],
            pointer: 0,
        }
    }

    fn read_current_cell(&self) -> u8 {
        self.memory[self.pointer]
    }

    fn write_current_cell(&mut self, value: u8) {
        self.memory[self.pointer] = value;
    }

    pub fn run(
        &mut self,
        program: &[Instruction],
        input: &mut dyn io::Read,
        output: &mut dyn io::Write,
    ) -> Result<(), Error> {
        let mut next_instruction = 0;

        while next_instruction < program.len() {
            match program[next_instruction] {
                Instruction::MoveRight(steps) => {
                    if self.pointer + steps >= self.memory.len() {
                        return Err(Error::PointerOverflow);
                    }
                    self.pointer += steps;
                }
                Instruction::MoveLeft(steps) => {
                    if self.pointer < steps {
                        return Err(Error::PointerUnderflow);
                    }
                    self.pointer -= steps;
                }
                Instruction::Add(term) => {
                    let value = self.read_current_cell().wrapping_add(term as u8);
                    self.write_current_cell(value);
                }
                Instruction::Sub(term) => {
                    let value = self.read_current_cell().wrapping_sub(term as u8);
                    self.write_current_cell(value);
                }
                Instruction::JumpIfZero(index) => {
                    if self.read_current_cell() == 0 {
                        next_instruction = index;
                    }
                }
                Instruction::JumpIfNotZero(index) => {
                    if self.read_current_cell() != 0 {
                        next_instruction = index;
                    }
                }
                Instruction::Print => {
                    output.write_all(&[self.read_current_cell()])?;
                }
                Instruction::Read => {
                    let mut buffer = [0; 1];
                    let bytes = input.read(&mut buffer)?;
                    let value = if bytes > 0 { buffer[0] } else { 0 };
                    self.write_current_cell(value);
                }
                Instruction::Clear => {
                    self.write_current_cell(0);
                }
            }
            next_instruction += 1;
        }
        Ok(())
    }
}
