use std::env;
use std::fs;
use std::io;
use std::time::Instant;

use brainrust::interpreter;
use brainrust::interpreter::Interpreter;
use brainrust::lexer;
use brainrust::optimizer;
use brainrust::parser;
use clap::Arg;
use clap::ArgAction;
use clap::ArgMatches;
use clap::Command;
use clap::crate_name;
use clap::crate_version;
use clap::value_parser;

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
    let matches = Command::new(crate_name!())
        .version(crate_version!())
        .long_version(crate_version!())
        .about("Brainfuck interpreter")
        .arg_required_else_help(true)
        .subcommand(
            Command::new(CLI_SUB_CMD_RUN)
                .about("Parses Brainfuck in the specified file and interprets it")
                .arg(
                    Arg::new(CLI_ARG_INPUT_FILE)
                        .help("The file to parse")
                        .index(1)
                        .required(true),
                )
                .arg(
                    Arg::new(CLI_SUB_CMD_RUN_MEMORY)
                        .help(&format!(
                            "Sets the number of memory cells, defaults to {:?}",
                            DEFAULT_MEMORY_SIZE
                        ))
                        .long(CLI_SUB_CMD_RUN_MEMORY)
                        .value_parser(value_parser!(u32)),
                )
                .arg(
                    Arg::new(CLI_SUB_CMD_RUN_TIME)
                        .help(&"Prints time of various metrics".to_string())
                        .long(CLI_SUB_CMD_RUN_TIME)
                        .value_parser(CLI_SUB_CMD_RUN_TIME_OPTIONS)
                        .action(ArgAction::Append),
                ),
        )
        .get_matches();

    match matches.subcommand() {
        Some((CLI_SUB_CMD_RUN, matches)) => handle_run(matches),
        _ => Err(CliError::ValidationError(String::from(
            "Invalid subcommand",
        ))),
    }
}

fn handle_run(matches: &ArgMatches) -> Result<(), CliError> {
    match matches.get_one::<String>(CLI_ARG_INPUT_FILE) {
        Some(input) => {
            let memory = *matches
                .get_one(CLI_SUB_CMD_RUN_MEMORY)
                .unwrap_or(&DEFAULT_MEMORY_SIZE);

            let time_metrics: Vec<&str> = matches
                .get_many::<String>(CLI_SUB_CMD_RUN_TIME)
                .unwrap_or_default()
                .map(String::as_str)
                .collect();

            let total_start = Instant::now();
            let contents = fs::read_to_string(input)?;
            let tokens = lexer::lex(&contents);
            let parsed = parser::parse(tokens)?;
            let optimized = optimizer::optimize(parsed);
            let parse_elapsed = total_start.elapsed();

            let mut interpreter = Interpreter::new(memory);

            let exec_start = Instant::now();
            interpreter.run(optimized, &mut io::stdin(), &mut io::stdout())?;
            let exec_elapsed = exec_start.elapsed();
            let total_elapsed = total_start.elapsed();

            if !time_metrics.is_empty() {
                println!();
                if time_metrics.contains(&"total") {
                    println!("Total time:     {}s", total_elapsed.as_secs());
                    println!("                {}ms", total_elapsed.as_millis());
                    println!("                {}µs", total_elapsed.as_micros());
                }
                if time_metrics.contains(&"exec") {
                    println!("Execution time: {}s", exec_elapsed.as_secs());
                    println!("                {}ms", exec_elapsed.as_millis());
                    println!("                {}µs", exec_elapsed.as_micros());
                }
                if time_metrics.contains(&"parse") {
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
