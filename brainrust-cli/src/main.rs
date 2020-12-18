use std::env;
use std::fs;
use std::io;
use std::process;
use std::time::Instant;

use brainrust_engine::interpreter;
use brainrust_engine::interpreter::Interpreter;
use brainrust_engine::lexer;
use brainrust_engine::optimizer;
use brainrust_engine::parser;

#[macro_use]
extern crate clap;
use clap::{App, AppSettings, Arg};

const DEFAULT_MEMORY_SIZE: usize = 32768;
const CLI_SUB_CMD_RUN: &str = "run";
const CLI_ARG_INPUT_FILE: &str = "input";
const CLI_SUB_CMD_RUN_MEMORY: &str = "memory";
const CLI_SUB_CMD_RUN_TIME: &str = "time";
const CLI_SUB_CMD_RUN_TIME_OPTIONS: [&str; 3] = ["total", "exec", "parse"];

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
    let matches = App::new("brainrust")
        .long_version(crate_version!())
        .version_short("v")
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
                )
                .arg(
                    Arg::with_name(CLI_SUB_CMD_RUN_MEMORY)
                        .help(&format!(
                            "Sets the number of memory cells, defaults to {:?}",
                            DEFAULT_MEMORY_SIZE
                        ))
                        .long(CLI_SUB_CMD_RUN_MEMORY)
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name(CLI_SUB_CMD_RUN_TIME)
                        .help(&"Prints time of various metrics".to_string())
                        .long(CLI_SUB_CMD_RUN_TIME)
                        .possible_values(&CLI_SUB_CMD_RUN_TIME_OPTIONS)
                        // .number_of_values(1)
                        .multiple(true)
                        .takes_value(true),
                ),
        )
        .get_matches();

    match matches.subcommand() {
        (CLI_SUB_CMD_RUN, Some(run)) => handle_run(run),
        _ => Err(CliError::ValidationError(String::from(
            "Invalid subcommand",
        ))),
    }
}

fn handle_run(matches: &clap::ArgMatches) -> Result<(), CliError> {
    match matches.value_of(CLI_ARG_INPUT_FILE) {
        Some(input) => {
            let mut memory = DEFAULT_MEMORY_SIZE;
            if let Some(exists) = matches.value_of(CLI_SUB_CMD_RUN_MEMORY) {
                if let Ok(value) = exists.parse::<usize>() {
                    memory = value;
                } else {
                    println!(
                        "Argument of {{{}}} could not be parsed",
                        CLI_SUB_CMD_RUN_MEMORY
                    );
                    process::exit(1);
                }
            }

            let total_start = Instant::now();
            let parse_start = Instant::now();
            let contents = fs::read_to_string(input)?;
            let tokens = lexer::lex(&contents);
            let parsed = parser::parse(tokens)?;
            let optimized = optimizer::optimize(parsed);
            let parse_elapsed = parse_start.elapsed();

            let mut interpreter = Interpreter::new(memory);

            let exec_start = Instant::now();
            interpreter.run(optimized, &mut io::stdin(), &mut io::stdout())?;
            let exec_elapsed = exec_start.elapsed();
            let total_elapsed = total_start.elapsed();

            if let Some(values) = matches.values_of(CLI_SUB_CMD_RUN_TIME) {
                let values: Vec<&str> = values.collect();
                println!();
                if values.contains(&"total") {
                    println!("Total time:     {}s", total_elapsed.as_secs());
                    println!("                {}ms", total_elapsed.as_millis());
                    println!("                {}µs", total_elapsed.as_micros());
                }
                if values.contains(&"exec") {
                    println!("Execution time: {}s", exec_elapsed.as_secs());
                    println!("                {}ms", exec_elapsed.as_millis());
                    println!("                {}µs", exec_elapsed.as_micros());
                }
                if values.contains(&"parse") {
                    println!("Parsing time:   {}s", parse_elapsed.as_secs());
                    println!("                {}ms", parse_elapsed.as_millis());
                    println!("                {}µs", parse_elapsed.as_micros());
                }
            }

            Ok(())
        }
        None => Err(CliError::ValidationError(String::from(
            "Input file missing",
        ))),
    }
}
