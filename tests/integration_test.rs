use brainrust::{
    interpreter::{self, Interpreter},
    lexer, optimizer, parser,
};
use std::{fs, io, str};

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
            let program_name = $program_name;
            let program = format!("{PROGRAM_PREFIX}{program_name}{PROGRAM_EXTENSION}");
            let input = format!("{PROGRAM_PREFIX}{program_name}{INPUT_EXTENSION}");
            let output = format!("{PROGRAM_PREFIX}{program_name}{OUTPUT_EXTENSION}");

            let program = fs::read_to_string(program)?;
            let input = fs::read_to_string(input)?;
            let output = fs::read(output)?;

            let result = run_program(&program, &input)?;

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
    let tokens = lexer::lex(file);
    let parsed = parser::parse(&tokens)?;
    let optimized = optimizer::optimize(parsed);
    let mut interpreter = Interpreter::new(MEMORY_SIZE);

    let mut output: Vec<u8> = vec![];

    interpreter.run(&optimized, &mut input.as_bytes(), &mut output)?;
    Ok(output)
}

#[derive(Debug)]
enum TestError {
    Io,
    Parsing,
    Interpreter,
    ConversationError,
}

impl From<io::Error> for TestError {
    fn from(_error: io::Error) -> Self {
        TestError::Io
    }
}

impl From<parser::Error> for TestError {
    fn from(_error: parser::Error) -> Self {
        TestError::Parsing
    }
}

impl From<interpreter::Error> for TestError {
    fn from(_error: interpreter::Error) -> Self {
        TestError::Interpreter
    }
}

impl From<str::Utf8Error> for TestError {
    fn from(_error: str::Utf8Error) -> Self {
        TestError::ConversationError
    }
}
