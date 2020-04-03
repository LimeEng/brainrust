![](https://github.com/LimeEng/brainrust/workflows/Rust/badge.svg)

# Brainrust

Brainrust is a simple Brainfuck interpreter written in Rust. [Brainfuck](https://en.wikipedia.org/wiki/Brainfuck) is a very simple but still Turing complete language with just eight instructions.

Brainrust consists of two parts, the CLI `brainrust-cli` and the engine `brainrust-engine`. The engine exposes methods to lex, parse, optimize and interpret Brainfuck code. The CLI is using the engine to interpret Brainfuck programs and is currently quite minimalistic.

This repository is a [Cargo workspace](https://doc.rust-lang.org/book/ch14-03-cargo-workspaces.html) which allows the library to evolve more independently from the CLI and vice versa.

- [Installation](#installation)
- [Usage](#usage)
- [Optimizations](#optimizations)
- [Resources](#resources)

## Installation

Install the CLI by running.
```
cargo install --git https://github.com/LimeEng/brainrust
```

It is also possible to download a pre-built binary for either Windows, Linux or macOS from the [releases-page](https://github.com/LimeEng/brainrust/releases).

## Usage

If you installed the CLI with cargo or downloaded a pre-built binary, just invoke the binary with a file as argument.
```
brainrust-cli file.b
```

If not, use cargo to run the CLI
```
cargo run --release file.b
```

## Optimizations

Below follows a list of optimizations that are currently implemented along with a short description.

**1. Statically linked loops**

It is possible to implement an interpreter that searches for matching brackets at runtime but this incurs a heavy performance hit. Brainrust instead statically links these at the parsing stage for `O(1)` access. The example belows illustrates a clear loop, that is, it sets the current cell to zero.

```
[ - ]
```

This loop gets parsed like this:
```
[ - ] => [JumpIfZero(2), Sub(1) JumpIfNotZero(0)]
```

The arguments to `JumpIfZero` and `JumpIfNotZero` specifies to which memory address the pointer should jump to, if the correct condition is fulfilled.

**2. Instruction stacking**

Some instructions can be stacked. These stackable instructions (`>`, `<`, `+`, `-`) can be combined to decrease the number of instructions, thus increasing performance. An example of instruction stacking can be found in the example below.

```
+++++ => [Add(1), Add(1), Add(1), Add(1), Add(1)] => [Add(5)]
```

**3. Clear loops**

The so called clear loop is a common idiom in Brainfuck. The purpose of the clear loop is to set the current cell to zero which is accomplished with the following loop.

```
[ - ] => [JumpIfZero(2), Sub(1) JumpIfNotZero(0)] => [Clear]
```

This can be optimized by replacing the loop with a single custom instruction `Clear`.

## Resources

Implementing optimized interpreters/compilers for Brainfuck is certainly nothing novel. Below are some useful resources on the topic.

- http://www.muppetlabs.com/~breadbox/bf/standards.html
- http://calmerthanyouare.org/2015/01/07/optimizing-brainfuck.html
- http://www.hevanet.com/cristofd/brainfuck/
- https://github.com/Wilfred/bfc
- https://www.nayuki.io/page/optimizing-brainfuck-compiler
- http://www.wilfred.me.uk/blog/2015/08/29/an-optimising-bf-compiler/

Additionally, here are some resources on where to find runnable Brainfuck programs.

- https://github.com/matslina/bfoptimization
- http://www.hevanet.com/cristofd/brainfuck/
- https://jonripley.com/i-fiction/games/LostKingdomBF.html
- http://www.linusakesson.net/programming/brainfuck/
