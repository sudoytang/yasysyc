# yasysyc

> This is a WIP.


Yet Another SysY Compiler - A compiler for the SysY programming language written in Rust.

## Overview

yasysyc is a compiler frontend for SysY, a simplified C-like language. It currently supports parsing SysY source code and generating an Abstract Syntax Tree (AST).

## Features

- LALRPOP-based parser generator
- AST generation and visualization

## Installation

### Prerequisites

- Rust toolchain (edition 2024)
- Cargo build system

### Build from Source

```bash
cargo build --release
```

The compiled binary will be located at `target/release/yasysyc`.

## Usage

### Basic Usage

```bash
yasysyc <input-file>
```

This will parse the input file and print the AST to stdout.

### Output to File

```bash
yasysyc <input-file> -o <output-file>
# or
yasysyc <input-file> --output <output-file>
```

### Debug Mode

```bash
yasysyc <input-file> --debug
```

Debug mode prints a detailed, formatted AST representation using Rust's `{:#?}` formatting.

### Examples

```bash
# Parse and print to stdout
yasysyc test.c

# Parse and save to file
yasysyc test.c -o output.txt

# Parse and print detailed debug AST
yasysyc test.c --debug
```

## Grammar (EBNF)

The current implementation supports a 'hello-world-like' minimal subset of SysY:

```ebnf
CompUnit      ::= FuncDef

FuncDef       ::= FuncType Ident "(" ")" Block

FuncType      ::= "int"

Block         ::= "{" Stmt "}"

Stmt          ::= "return" Number ";"

Number        ::= IntConst

Ident         ::= [_a-zA-Z][_a-zA-Z0-9]*

IntConst      ::= DecConst | OctConst | HexConst

DecConst      ::= [1-9][0-9]*

OctConst      ::= 0[0-7]*

HexConst      ::= 0[xX][0-9a-fA-F]+
```

### Lexical Rules

- **Whitespace**: Automatically skipped
- **Line Comments**: `// ...` (skipped)
- **Block Comments**: `/* ... */` (skipped)
- **Identifiers**: Must start with letter or underscore, followed by letters, digits, or underscores
- **Integer Literals**:
  - Decimal: `1-9` followed by `0-9*`
  - Octal: `0` followed by `0-7*`
  - Hexadecimal: `0x` or `0X` followed by hex digits

## Example Program

```c
int main() {
    // this is a line comment
    /* this is a block comment */
    return 0;
}
```

Output (normal mode):
```
int main() { return 0; }
```

## AST Structure

The AST consists of the following node types:

- `CompUnit`: Root node containing a function definition
- `FuncDef`: Function definition with type, identifier, and block
- `FuncType`: Currently only `Int` is supported
- `Block`: Contains a single statement
- `Stmt`: Statement enum (currently only `Return`)
- `ReturnStmt`: Return statement with a number
- `Number`: Integer value wrapper
- `Ident`: Identifier wrapper

All AST nodes implement `Display` for pretty-printing and `Debug` for detailed inspection.

## Dependencies

- [lalrpop](https://github.com/lalrpop/lalrpop) - Parser generator
- [clap](https://github.com/clap-rs/clap) - Command-line argument parser
- [anyhow](https://github.com/dtolnay/anyhow) - Error handling
- [koopa](https://github.com/pku-minic/koopa) - Koopa IR support (planned)

## Current Limitations

- Only supports a single function definition per compilation unit
- Only `int` return type is supported
- Only `return` statements with integer literals are supported
- No support for:
  - Variables
  - Expressions
  - Control flow statements
  - Multiple functions
  - Function parameters
  - Global declarations

## Development

### Building

```bash
cargo build
```

### Running Tests

```bash
cargo test
```

### Generating Parser

The parser is automatically generated from `src/sysy.lalrpop` during the build process via the `build.rs` script.

## Current Progress

### Implemented

- ✅ Project structure and build system setup
- ✅ LALRPOP parser integration
- ✅ Basic lexical analysis
  - Whitespace and comment skipping (line comments `//`, block comments `/* */`)
  - Identifier recognition
  - Integer literal parsing (decimal, octal, hexadecimal)
- ✅ AST data structures
  - `CompUnit`, `FuncDef`, `FuncType`, `Block`, `Stmt`, `ReturnStmt`, `Number`, `Ident`
  - `Display` trait implementation for AST pretty-printing
  - `Debug` trait for detailed AST inspection
- ✅ Minimal grammar support
  - Single function definition with `int` return type
  - Parameter-less function
  - Single return statement with integer literal
- ✅ CLI interface
  - Input file parsing
  - Optional output file
  - Debug mode for AST visualization

### Not Yet Implemented

- ❌ Expression parsing (arithmetic, logical, relational)
- ❌ Variable declarations and references
- ❌ Function parameters and arguments
- ❌ Control flow statements (if, while, for, break, continue)
- ❌ Multiple statements in a block
- ❌ Multiple function definitions
- ❌ Global variable declarations
- ❌ Arrays
- ❌ IR generation
- ❌ Code generation

## Changelog

### v0.1.0 (Initial Implementation)

**Added:**
- Basic project structure with Cargo configuration
- LALRPOP parser generator integration via `build.rs`
- Lexer rules for comments, identifiers, and integer literals
- Minimal SysY grammar supporting single-function programs
- AST node definitions with display formatting
- CLI tool with clap for command-line argument parsing
- Debug mode for detailed AST output
- Support for decimal, octal, and hexadecimal integer literals
