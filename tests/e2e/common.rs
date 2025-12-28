//! Common utilities for E2E differential tests

use std::error::Error;
use std::fmt;

/// Errors that can occur during testing
#[derive(Debug)]
pub enum TestError {
    /// I/O error
    Io(String),
    /// Reference implementation failed
    Reference(String),
    /// Compilation failed (yasysyc)
    Compile(String),
    /// Assembly failed (riscv-gcc)
    Assemble(String),
    /// Runtime error
    Run(String),
    /// Result mismatch between reference and test
    Mismatch {
        expected: i32,
        actual: i32,
        asm: String,
    },
}

impl fmt::Display for TestError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TestError::Io(msg) => write!(f, "I/O error: {}", msg),
            TestError::Reference(msg) => write!(f, "Reference implementation error: {}", msg),
            TestError::Compile(msg) => write!(f, "Compilation error (yasysyc): {}", msg),
            TestError::Assemble(msg) => write!(f, "Assembly error (riscv-gcc): {}", msg),
            TestError::Run(msg) => write!(f, "Runtime error: {}", msg),
            TestError::Mismatch { expected, actual, asm } => {
                write!(
                    f,
                    "Result mismatch:\n  GCC reference exit code: {}\n  yasysyc exit code: {}\n\nGenerated assembly:\n{}",
                    expected,
                    actual,
                    asm
                )
            }
        }
    }
}

impl Error for TestError {}
