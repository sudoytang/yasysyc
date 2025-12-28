# yasysyc

> This is a WIP.


Yet Another SysY Compiler - A compiler for the SysY programming language written in Rust.

## Overview

yasysyc is a compiler for SysY, a simplified C-like language. It supports parsing SysY source code, generating Koopa IR, and compiling to RISC-V assembly.

## Features

- LALRPOP-based parser generator
- AST generation and visualization
- Koopa IR generation
- RISC-V assembly code generation
- Full expression support (unary, binary, comparison, logical operators)

## Installation

### Prerequisites

- Rust toolchain (edition 2024)
- Cargo build system
- (Optional) RISC-V toolchain for running generated assembly

### Build from Source

```bash
cargo build --release
```

The compiled binary will be located at `target/release/yasysyc`.

## Usage

### Generate Koopa IR

```bash
yasysyc --koopa <input-file> -o <output-file>
```

### Generate RISC-V Assembly

```bash
yasysyc --riscv <input-file> -o <output-file>
```

### Output to stdout

```bash
# Koopa IR to stdout
yasysyc --koopa <input-file>

# RISC-V assembly to stdout
yasysyc --riscv <input-file>
```

### Examples

```bash
# Generate Koopa IR
yasysyc --koopa test.c -o test.koopa

# Generate RISC-V assembly
yasysyc --riscv test.c -o test.S

# Print RISC-V assembly to stdout
yasysyc --riscv test.c
```

## Supported Operators

### Unary Operators
| Operator | Description |
|----------|-------------|
| `+` | Unary plus |
| `-` | Unary minus (negation) |
| `!` | Logical NOT |

### Binary Operators
| Operator | Description |
|----------|-------------|
| `+` | Addition |
| `-` | Subtraction |
| `*` | Multiplication |
| `/` | Division |
| `%` | Modulo |

### Comparison Operators
| Operator | Description |
|----------|-------------|
| `<` | Less than |
| `>` | Greater than |
| `<=` | Less than or equal |
| `>=` | Greater than or equal |
| `==` | Equal |
| `!=` | Not equal |

### Logical Operators
| Operator | Description |
|----------|-------------|
| `&&` | Logical AND |
| `\|\|` | Logical OR |

## Operator Precedence (lowest to highest)

1. `||` (Logical OR)
2. `&&` (Logical AND)
3. `==`, `!=` (Equality)
4. `<`, `>`, `<=`, `>=` (Relational)
5. `+`, `-` (Additive)
6. `*`, `/`, `%` (Multiplicative)
7. `+`, `-`, `!` (Unary)

## Grammar (EBNF)

```ebnf
CompUnit      ::= FuncDef

FuncDef       ::= FuncType Ident "(" ")" Block

FuncType      ::= "int"

Block         ::= "{" Stmt "}"

Stmt          ::= "return" Expr ";"

Expr          ::= LogicOrExpr

LogicOrExpr   ::= LogicAndExpr | LogicOrExpr "||" LogicAndExpr

LogicAndExpr  ::= EqExpr | LogicAndExpr "&&" EqExpr

EqExpr        ::= RelExpr | EqExpr ("==" | "!=") RelExpr

RelExpr       ::= AddExpr | RelExpr ("<" | ">" | "<=" | ">=") AddExpr

AddExpr       ::= MulExpr | AddExpr ("+" | "-") MulExpr

MulExpr       ::= UnaryExpr | MulExpr ("*" | "/" | "%") UnaryExpr

UnaryExpr     ::= PrimaryExpr | ("+" | "-" | "!") UnaryExpr

PrimaryExpr   ::= Number | "(" Expr ")"

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
    return 1 + 2 * 3 < 10 && 5 >= 5;
}
```

Generated RISC-V assembly:
```asm
.text
.globl main
main:
  addi sp, sp, -32
  li t0, 2
  li t1, 3
  mul t2, t0, t1
  sw t2, 0(sp)
  li t0, 1
  lw t1, 0(sp)
  add t2, t0, t1
  sw t2, 4(sp)
  lw t0, 4(sp)
  li t1, 10
  slt t2, t0, t1
  ...
```

## Code Generation Notes

### RISC-V Implementation Details

Since RISC-V only provides `slt` (set less than) for comparison, other comparisons are implemented as:

| Operation | RISC-V Implementation |
|-----------|----------------------|
| `a < b` | `slt rd, a, b` |
| `a > b` | `slt rd, b, a` (swap operands) |
| `a <= b` | `slt rd, b, a; xori rd, rd, 1` |
| `a >= b` | `slt rd, a, b; xori rd, rd, 1` |
| `a == b` | `sub rd, a, b; seqz rd, rd` |
| `a != b` | `sub rd, a, b; snez rd, rd` |

Logical operators return 0 or 1:
- `a && b` → `snez t0, a; snez t1, b; and rd, t0, t1`
- `a || b` → `or rd, a, b; snez rd, rd`

### Koopa IR Notes

Koopa IR only supports bitwise AND/OR, so logical operators are transformed:
- `a || b` → `(a | b) != 0`
- `a && b` → `(a != 0) & (b != 0)`

## Dependencies

- [lalrpop](https://github.com/lalrpop/lalrpop) - Parser generator
- [clap](https://github.com/clap-rs/clap) - Command-line argument parser
- [anyhow](https://github.com/dtolnay/anyhow) - Error handling
- [koopa](https://github.com/pku-minic/koopa) - Koopa IR library

## Current Limitations

- Only supports a single function definition per compilation unit
- Only `int` return type is supported
- Only `return` statements are supported (no other control flow)
- No support for:
  - Variables
  - Control flow statements (if, while, for)
  - Multiple functions
  - Function parameters
  - Global declarations
  - Arrays

## Development

### Building

```bash
cargo build
```

### Running Tests

```bash
cargo test
```

The E2E tests use differential testing against GCC/spike to verify correctness.

### Generating Parser

The parser is automatically generated from `src/sysy.lalrpop` during the build process via the `build.rs` script.

## Current Progress

### Implemented

- ✅ Project structure and build system setup
- ✅ LALRPOP parser integration
- ✅ Lexical analysis (whitespace, comments, identifiers, integers)
- ✅ AST data structures with Display/Debug traits
- ✅ Unary expressions (`+`, `-`, `!`)
- ✅ Binary expressions (`+`, `-`, `*`, `/`, `%`)
- ✅ Comparison expressions (`<`, `>`, `<=`, `>=`, `==`, `!=`)
- ✅ Logical expressions (`&&`, `||`)
- ✅ Parenthesized expressions
- ✅ Operator precedence
- ✅ Koopa IR generation
- ✅ RISC-V assembly generation
- ✅ Stack-based register allocation
- ✅ E2E test framework

### Not Yet Implemented

- ❌ Variable declarations and references
- ❌ Function parameters and arguments
- ❌ Control flow statements (if, while, for, break, continue)
- ❌ Multiple statements in a block
- ❌ Multiple function definitions
- ❌ Global variable declarations
- ❌ Arrays
- ❌ Advanced register allocation
