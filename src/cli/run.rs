use super::CliError;
use crate::{cli::util, interpreter, program::Program};
use clap::{Arg, ArgAction, ArgMatches, Command, value_parser};
use std::{fs, io, time::Instant};

const DEFAULT_MEMORY_SIZE: usize = 32768;
const ARG_INPUT_FILE: &str = "input";
const ARG_MEMORY: &str = "memory";
const ARG_RUN_TIME: &str = "time";
const ARG_RUN_TIME_OPTIONS: [&str; 3] = ["total", "exec", "parse"];

pub fn build_command() -> Command {
    Command::new("run")
        .about("Parses Brainfuck in the specified file and interprets it")
        .arg(
            Arg::new(ARG_INPUT_FILE)
                .help("The file to parse")
                .index(1)
                .required(true),
        )
        .arg(
            Arg::new(ARG_MEMORY)
                .help(format!(
                    "Sets the number of memory cells, defaults to {DEFAULT_MEMORY_SIZE:?}"
                ))
                .long(ARG_MEMORY)
                .value_parser(value_parser!(u32)),
        )
        .arg(
            Arg::new(ARG_RUN_TIME)
                .help("Prints time of various metrics")
                .long(ARG_RUN_TIME)
                .value_parser(ARG_RUN_TIME_OPTIONS)
                .action(ArgAction::Append),
        )
}

pub fn execute(matches: &ArgMatches) -> Result<(), CliError> {
    let input_file = matches
        .get_one::<String>(ARG_INPUT_FILE)
        .expect("Input file is required");
    let memory = *matches.get_one(ARG_MEMORY).unwrap_or(&DEFAULT_MEMORY_SIZE);
    let time_metrics: Vec<&str> = matches
        .get_many::<String>(ARG_RUN_TIME)
        .unwrap_or_default()
        .map(String::as_str)
        .collect();

    let total_start = Instant::now();
    let contents = fs::read_to_string(input_file)?;
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
        if time_metrics.contains(&"parse") {
            println!("Parsing time:   {}", util::format_duration(parse_elapsed));
        }
        if time_metrics.contains(&"exec") {
            println!("Execution time: {}", util::format_duration(exec_elapsed));
        }
        if time_metrics.contains(&"total") {
            println!("Total time:     {}", util::format_duration(total_elapsed));
        }
    }
    Ok(())
}
