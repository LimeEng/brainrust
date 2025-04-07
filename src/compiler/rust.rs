use crate::program::{Instruction, Program};
use std::fmt::Write;

const VAR_MEMORY: &str = "memory";
const VAR_POINTER: &str = "pointer";
const FUN_READ: &str = "read";
const FUN_PRINT: &str = "print";
const MEMORY_SIZE: usize = 32768;

struct CodeGen {
    source: String,
    indent: usize,
}

impl CodeGen {
    fn new() -> Self {
        Self {
            source: String::new(),
            indent: 0,
        }
    }

    fn line(&mut self, code: impl AsRef<str>) -> &mut Self {
        let indent = "    ".repeat(self.indent);
        let code = code.as_ref();
        writeln!(&mut self.source, "{indent}{code}").unwrap();
        self
    }

    fn empty_line(&mut self) -> &mut Self {
        self.source.push('\n');
        self
    }

    fn block<F>(&mut self, header: impl AsRef<str>, body: F) -> &mut Self
    where
        F: FnOnce(&mut Self),
    {
        let header = header.as_ref();
        self.line(format!("{header} {{"));
        self.indent += 1;
        body(self);
        self.indent -= 1;
        self.line("}")
    }

    fn function<F>(
        &mut self,
        name: &str,
        params: Option<&[&str]>,
        return_type: Option<&str>,
        body: F,
    ) -> &mut Self
    where
        F: FnOnce(&mut Self),
    {
        let params = params.map(|params| params.join(", ")).unwrap_or_default();
        let header = match return_type {
            Some(return_type) => format!("fn {name}({params}) -> {return_type}"),
            None => format!("fn {name}({params})"),
        };
        self.block(header, |code| body(code))
    }

    fn gen_instruction(&mut self, instruction: &Instruction) -> &mut Self {
        macro_rules! mem {
            (@) => {
                format!("{VAR_MEMORY}[{VAR_POINTER}]")
            };
        }

        use Instruction as Instr;
        match instruction {
            Instr::MoveRight(value) => self.line(format!("{VAR_POINTER} += {value};")),
            Instr::MoveLeft(value) => self.line(format!("{VAR_POINTER} -= {value};")),
            Instr::Add(value) => {
                self.line(format!("{} = {}.wrapping_add({value});", mem!(@), mem!(@)))
            }
            Instr::Sub(value) => {
                self.line(format!("{} = {}.wrapping_sub({value});", mem!(@), mem!(@)))
            }
            Instr::Print => self.line(format!("{FUN_PRINT}({});", mem!(@))),
            Instr::Read => self.line(format!("{} = {FUN_READ}();", mem!(@))),
            Instr::Set(value) => self.line(format!("{} = {value};", mem!(@))),
            Instr::Loop { body } => self.block(format!("while {} != 0", mem!(@)), |code| {
                for instr in body {
                    code.gen_instruction(instr);
                }
            }),
        }
    }

    fn build(self) -> String {
        self.source
    }
}

pub fn generate_code(program: &Program) -> String {
    let mut code = CodeGen::new();

    code.line("use std::io::{self, Read, Write};")
        .empty_line()
        .function("read", None, Some("u8"), |code| {
            code.line("let mut buffer = [0; 1];")
                .line("let bytes = io::stdin().read(&mut buffer).unwrap();")
                .line("if bytes > 0 { buffer[0] } else { 0 }");
        })
        .empty_line()
        .function("print", Some(&["value: u8"]), None, |code| {
            code.line("io::stdout().write_all(&[value]).unwrap();");
        })
        .empty_line()
        .function("main", None, None, |code| {
            code.line(format!("let mut memory = [0u8; {MEMORY_SIZE}];"))
                .line("let mut pointer = 0usize;")
                .empty_line();

            for instruction in program.instructions() {
                code.gen_instruction(instruction);
            }
        });

    code.build()
}
