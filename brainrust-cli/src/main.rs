use std::env;
use std::fs;
use std::io;
use std::time::Instant;

use brainrust_engine::interpreter;
use brainrust_engine::interpreter::Interpreter;
use brainrust_engine::lexer;
use brainrust_engine::optimizer;
use brainrust_engine::parser;

#[macro_use]
extern crate clap;
use clap::{App, AppSettings, Arg};

const MEMORY_SIZE: usize = 32768;
const CLI_SUB_CMD_RUN: &str = "run";
const CLI_ARG_INPUT_FILE: &str = "input";

#[derive(Debug)]
pub enum CliError {
    IoError(std::io::Error),
    ParsingError(parser::Error),
    InterpreterError(interpreter::Error),
    ValidationError(String),
}

impl From<std::io::Error> for CliError {
    fn from(io_error: std::io::Error) -> Self {
        CliError::IoError(io_error)
    }
}

impl From<parser::Error> for CliError {
    fn from(parser_error: parser::Error) -> Self {
        CliError::ParsingError(parser_error)
    }
}

impl From<interpreter::Error> for CliError {
    fn from(interpreter_error: interpreter::Error) -> Self {
        CliError::InterpreterError(interpreter_error)
    }
}

fn main() -> Result<(), CliError> {
    let matches = App::new("Brainrust")
        .version(crate_version!())
        .author("Emil E")
        .about("Interprets Brainfuck efficiently")
        .setting(AppSettings::ArgRequiredElseHelp)
        .subcommand(
            App::new(CLI_SUB_CMD_RUN)
                .about("Parses Brainfuck in the specified file and interprets it")
                .arg(
                    Arg::with_name(CLI_ARG_INPUT_FILE)
                        .help("The file to parse")
                        .index(1)
                        .required(true),
                ),
        )
        .get_matches();

    match matches.subcommand() {
        (CLI_SUB_CMD_RUN, Some(run)) => handle_run(run),
        // TODO: This case will never happen as Clap handles it. Should this still be here?
        _ => Err(CliError::ValidationError(String::from(
            "Invalid subcommand",
        ))),
    }
}

fn handle_run(matches: &clap::ArgMatches) -> Result<(), CliError> {
    match matches.value_of(CLI_ARG_INPUT_FILE) {
        Some(input) => {
            let contents = fs::read_to_string(input)?;
            let tokens = lexer::lex(&contents);
            let parsed = parser::parse(tokens)?;
            let optimized = optimizer::optimize(parsed);

            let mut interpreter = Interpreter::new(MEMORY_SIZE);

            let now = Instant::now();
            interpreter.run(optimized, &mut io::stdin(), &mut io::stdout())?;
            let elapsed = now.elapsed();

            println!();
            println!("Execution time: {}s", elapsed.as_secs());
            println!("                {}ms", elapsed.as_millis());
            println!("                {}µs", elapsed.as_micros());
            Ok(())
        }
        // TODO: This case will never happen as the CLI_ARG_INPUT_FILE is marked as required. Should this still be here?
        None => Err(CliError::ValidationError(String::from(
            "Input file missing",
        ))),
    }
}
