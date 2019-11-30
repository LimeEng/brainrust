use std::env;
use std::fs;
use std::io;
use std::time::Instant;

use brainrust_engine::interpreter::Interpreter;
use brainrust_engine::lexer;
use brainrust_engine::optimizer;
use brainrust_engine::parser;

const MEMORY_SIZE: usize = 32768;

fn main() {
    let args: Vec<String> = env::args().collect();

    let file = &args[1];

    println!("About to parse file {}...", file);

    let contents = fs::read_to_string(file).expect("Something went wrong reading the file");

    let tokens = lexer::lex(&contents);
    let parsed = parser::parse(tokens).expect("Parsing failure");
    let optimized = optimizer::optimize(parsed);

    println!("File parsed and optimized");

    let mut interpreter = Interpreter::new(MEMORY_SIZE);

    println!("Begin execution...");
    println!("==================");

    let now = Instant::now();
    let res = interpreter.run(optimized, &mut io::stdin(), &mut io::stdout());
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
