use brainrust::{
    interpreter,
    program::{Program, parser},
};

const MEMORY_SIZE: usize = 32768;

macro_rules! file_path {
    ($program:ident, $ext:literal) => {
        concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/tests/programs/",
            stringify!($program),
            $ext
        )
    };
}

macro_rules! include_file {
    (string, $program:ident, $ext:literal) => {
        include_str!(file_path!($program, $ext))
    };
    (bytes, $program:ident, $ext:literal) => {
        include_bytes!(file_path!($program, $ext))
    };
}

macro_rules! test_programs {
    ($($program:ident,)*) => {
    $(
        paste::item! {
            #[test]
            fn [< test_ $program >] () -> Result<(), TestError> {
                let program = include_file!(string, $program, ".b");
                let input = include_file!(string, $program, ".input");
                let output = include_file!(bytes, $program, ".output");

                let result = run_program(program, input)?;

                assert_eq!(result, output);
                Ok(())
            }
        }
    )*
    }
}

test_programs! {
    monty,
}

fn run_program(file: &str, input: &str) -> Result<Vec<u8>, TestError> {
    let mut input = input.as_bytes();
    let mut output: Vec<u8> = vec![];
    let mut tape = interpreter::Tape::new(&mut input, &mut output, MEMORY_SIZE);

    let program = Program::parse(file)?;
    let program = program.optimized();

    tape.execute(&program)?;

    Ok(output)
}

#[derive(Debug)]
enum TestError {
    Parsing,
    Interpreter,
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
