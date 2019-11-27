pub enum Instruction {
    MoveRight(usize),
    MoveLeft(usize),
    Add(usize),
    Sub(usize),
    JumpIfZero(usize),
    JumpIfNotZero(usize),
    Print,
    Read,
}
