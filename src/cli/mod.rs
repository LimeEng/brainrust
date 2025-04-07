use crate::{interpreter, program};
use clap::{Command, crate_name, crate_version};
use std::{env, io};

mod run;
mod util;

pub fn run() -> Result<(), Error> {
    let matches = Command::new(crate_name!())
        .version(crate_version!())
        .about("Brainfuck interpreter")
        .arg_required_else_help(true)
        .subcommand_required(true)
        .subcommand(run::build_command())
        .get_matches();

    match matches.subcommand() {
        Some(("run", matches)) => run::execute(matches),
        _ => unreachable!(),
    }
}

#[derive(Debug)]
pub enum Error {
    Io(io::Error),
    Parsing(program::Error),
    Interpreter(interpreter::Error),
}

impl From<io::Error> for Error {
    fn from(error: io::Error) -> Self {
        Error::Io(error)
    }
}

impl From<program::Error> for Error {
    fn from(error: program::Error) -> Self {
        Error::Parsing(error)
    }
}

impl From<interpreter::Error> for Error {
    fn from(error: interpreter::Error) -> Self {
        Error::Interpreter(error)
    }
}
