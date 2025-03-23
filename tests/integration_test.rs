use brainrust::interpreter;
use brainrust::interpreter::Interpreter;
use brainrust::lexer;
use brainrust::optimizer;
use brainrust::parser;

use std::fs;
use std::str;

const PROGRAM_PREFIX: &str = "tests/programs/";
const PROGRAM_EXTENSION: &str = ".b";
const INPUT_EXTENSION: &str = ".input";
const OUTPUT_EXTENSION: &str = ".output";

const MEMORY_SIZE: usize = 32768;

macro_rules! test_programs {
    ($($test_name:ident: $program_name:expr,)*) => {
    $(
        #[test]
        fn $test_name() -> Result<(), TestError> {
            let program = format!("{}{}{}", PROGRAM_PREFIX, $program_name, PROGRAM_EXTENSION);
            let input = format!("{}{}{}", PROGRAM_PREFIX, $program_name, INPUT_EXTENSION);
            let output = format!("{}{}{}", PROGRAM_PREFIX, $program_name, OUTPUT_EXTENSION);

            let program = fs::read(program)?;
            let input = fs::read(input)?;
            let output = fs::read(output)?;

            let program = str::from_utf8(&program)?;
            let input = str::from_utf8(&input)?;

            let result = run_program(program, input)?;

            assert_eq!(result, output);
            Ok(())
        }
    )*
    }
}

test_programs! {
    monty: "monty",
}

fn run_program(file: &str, input: &str) -> Result<Vec<u8>, TestError> {
    let tokens = lexer::lex(&file);
    let parsed = parser::parse(tokens)?;
    let optimized = optimizer::optimize(parsed);
    let mut interpreter = Interpreter::new(MEMORY_SIZE);

    let mut output: Vec<u8> = vec![];

    interpreter.run(optimized, &mut input.as_bytes(), &mut output)?;
    Ok(output)
}

#[derive(Debug)]
enum TestError {
    Io(std::io::Error),
    Parsing(parser::Error),
    Interpreter(interpreter::Error),
    ConversationError(std::str::Utf8Error),
}

impl From<std::io::Error> for TestError {
    fn from(io_error: std::io::Error) -> Self {
        TestError::Io(io_error)
    }
}

impl From<parser::Error> for TestError {
    fn from(parser_error: parser::Error) -> Self {
        TestError::Parsing(parser_error)
    }
}

impl From<interpreter::Error> for TestError {
    fn from(interpreter_error: interpreter::Error) -> Self {
        TestError::Interpreter(interpreter_error)
    }
}

impl From<std::str::Utf8Error> for TestError {
    fn from(utf8_error: std::str::Utf8Error) -> Self {
        TestError::ConversationError(utf8_error)
    }
}
