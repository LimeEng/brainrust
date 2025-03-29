use crate::{interpreter, program};
use clap::{Command, crate_name, crate_version};
use std::env;

mod run;
mod util;

#[derive(Debug)]
pub enum CliError {
    IoError(std::io::Error),
    ParsingError(program::Error),
    Interpreter(interpreter::Error),
}

impl From<std::io::Error> for CliError {
    fn from(error: std::io::Error) -> Self {
        CliError::IoError(error)
    }
}

impl From<program::Error> for CliError {
    fn from(error: program::Error) -> Self {
        CliError::ParsingError(error)
    }
}

impl From<interpreter::Error> for CliError {
    fn from(error: interpreter::Error) -> Self {
        CliError::Interpreter(error)
    }
}

pub fn run() -> Result<(), CliError> {
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
