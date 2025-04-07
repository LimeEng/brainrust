use crate::{
    cli::util,
    interpreter::{self, Analytics},
    program::Program,
};
use clap::{Arg, ArgAction, ArgMatches, Command, value_parser};
use std::{fs, io, time::Instant};

const DEFAULT_MEMORY_SIZE: &str = "32768";
const ARG_INPUT_FILE: &str = "input";
const ARG_MEMORY_SIZE: &str = "memory";
const ARG_TIME: &str = "time";
const ARG_PROFILE: &str = "profile";

pub fn build_command() -> Command {
    Command::new("run")
        .about("Parse and execute a Brainfuck program from a file")
        .arg(
            Arg::new(ARG_INPUT_FILE)
                .help("Path to the Brainfuck source file")
                .index(1)
                .required(true),
        )
        .arg(
            Arg::new(ARG_MEMORY_SIZE)
                .help("Number of memory cells")
                .long(ARG_MEMORY_SIZE)
                .action(ArgAction::Set)
                .default_value(DEFAULT_MEMORY_SIZE)
                .value_parser(value_parser!(usize)),
        )
        .arg(
            Arg::new(ARG_PROFILE)
                .help("Collect and print program metrics")
                .long_help("Collect and print program metrics. Substantially increases execution time and memory usage.")
                .long(ARG_PROFILE)
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new(ARG_TIME)
                .help("Print parsing and execution time")
                .long(ARG_TIME)
                .action(ArgAction::SetTrue),
        )
}

pub fn execute(matches: &ArgMatches) -> Result<(), crate::cli::Error> {
    let input_file = matches
        .get_one::<String>(ARG_INPUT_FILE)
        .expect("Input file is required");
    let memory_size = *matches
        .get_one(ARG_MEMORY_SIZE)
        .expect("Memory size should have a default value");
    let should_profile = *matches.get_one::<bool>(ARG_PROFILE).unwrap_or(&false);
    let print_timings = *matches.get_one::<bool>(ARG_TIME).unwrap_or(&false);

    let start = Instant::now();
    let contents = fs::read_to_string(input_file)?;
    let program = Program::parse(&contents)?;
    let program = program.optimized();
    let parse_elapsed = util::format_duration(start.elapsed());

    let mut input = io::stdin();
    let mut output = io::stdout();

    let start = Instant::now();
    let exec_elapsed = if should_profile {
        let analytics = interpreter::profile(&program, &mut input, &mut output, memory_size)?;
        print_analytics(&analytics);
        util::format_duration(start.elapsed())
    } else {
        interpreter::execute(&program, &mut input, &mut output, memory_size)?;
        util::format_duration(start.elapsed())
    };

    if print_timings {
        println!();
        println!("Parsing time:   {parse_elapsed}");
        println!("Execution time: {exec_elapsed}");
    }
    Ok(())
}

fn print_analytics(analytics: &Analytics) {
    let freq_table = util::build_frequency_table(analytics);
    let loop_table = util::build_loop_patterns_table(analytics);
    let misc_table = util::build_misc_table(analytics);

    println!();
    println!("{freq_table}");
    println!();
    println!("{loop_table}");
    println!();
    println!("{misc_table}");
}
