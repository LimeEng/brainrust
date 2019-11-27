use std::env;
use std::fs;
use std::time::Instant;

use std::io;

use brainrust_engine::interpreter::Interpreter;
use brainrust_engine::parser;

const MEMORY_SIZE: usize = 32768;

fn main() {
    let args: Vec<String> = env::args().collect();

    let file = &args[1];

    println!("About to parse file {}...", file);

    let contents = fs::read_to_string(file).expect("Something went wrong reading the file");

    let parsed = parser::parse(&contents).expect("Parsing failure");

    println!("File parsed");

    let mut interpreter = Interpreter::new(MEMORY_SIZE);

    println!("Begin execution...");
    println!("==================");

    let now = Instant::now();
    let res = interpreter.run(parsed, &mut io::stdin(), &mut io::stdout());
    let elapsed = now.elapsed();

    println!("==================");

    match res {
        Ok(_) => println!("Program finished successfully!"),
        Err(e) => println!("Something went wrong {:?}", e),
    }
    println!("Execution time: {}s", elapsed.as_secs());
    println!("                {}ms", elapsed.as_millis());
    println!("                {}µs", elapsed.as_micros());
}
