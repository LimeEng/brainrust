use crate::program::Program;
use std::{fs, io};

mod rust;

pub fn compile(program: &Program) -> io::Result<()> {
    let code = rust::generate_code(program);
    fs::write("compiled_bf.rs", code)
}
