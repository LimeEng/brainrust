use brainrust::{
    interpreter,
    program::{Program, parser},
};
use clap::{Arg, ArgAction, ArgMatches, Command, crate_name, crate_version, value_parser};
use std::{env, fs, io, time::Instant};

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
}

impl From<std::io::Error> for CliError {
    fn from(error: std::io::Error) -> Self {
        CliError::IoError(error)
    }
}

impl From<parser::Error> for CliError {
    fn from(error: parser::Error) -> Self {
        CliError::ParsingError(error)
    }
}

impl From<interpreter::Error> for CliError {
    fn from(error: interpreter::Error) -> Self {
        CliError::InterpreterError(error)
    }
}

fn main() -> Result<(), CliError> {
    let matches = Command::new(crate_name!())
        .version(crate_version!())
        .long_version(crate_version!())
        .about("Brainfuck interpreter")
        .arg_required_else_help(true)
        .subcommand_required(true)
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
                        .help(format!(
                            "Sets the number of memory cells, defaults to {DEFAULT_MEMORY_SIZE:?}"
                        ))
                        .long(CLI_SUB_CMD_RUN_MEMORY)
                        .value_parser(value_parser!(u32)),
                )
                .arg(
                    Arg::new(CLI_SUB_CMD_RUN_TIME)
                        .help("Prints time of various metrics")
                        .long(CLI_SUB_CMD_RUN_TIME)
                        .value_parser(CLI_SUB_CMD_RUN_TIME_OPTIONS)
                        .action(ArgAction::Append),
                ),
        )
        .get_matches();

    match matches.subcommand() {
        Some((CLI_SUB_CMD_RUN, matches)) => handle_run(matches),
        _ => unreachable!(),
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
            let program = Program::parse(&contents)?;
            let program = program.optimized();
            let parse_elapsed = total_start.elapsed();

            let mut input = io::stdin();
            let mut output = io::stdout();
            let mut tape = interpreter::Tape::new(&mut input, &mut output, memory);

            let exec_start = Instant::now();
            tape.execute(&program)?;
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
        None => unreachable!(),
    }
}
