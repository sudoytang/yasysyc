//! End-to-end test runner for yasysyc compiler
//!
//! Uses differential testing: compares yasysyc output against GCC reference implementation.
//! Each .c file in tests/e2e/cases/ becomes a separate test case.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

mod common;
use common::TestError;

/// Get the path to the yasysyc binary
fn get_compiler_path() -> PathBuf {
    // Try debug build first, then release
    let debug_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("target/debug/yasysyc");
    let release_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("target/release/yasysyc");

    if debug_path.exists() {
        debug_path
    } else if release_path.exists() {
        release_path
    } else {
        panic!("yasysyc binary not found. Run `cargo build` first.");
    }
}

/// Run a single test case using differential testing
fn run_test(source_path: &Path) -> datatest_stable::Result<()> {
    // Create temp directory for intermediate files
    let temp_dir = tempfile::tempdir()
        .map_err(|e| TestError::Io(format!("Failed to create temp dir: {}", e)))?;

    // ============================================================
    // Reference implementation: compile directly with GCC and run
    // ============================================================
    let ref_exe = temp_dir.path().join("ref");
    let ref_compile = Command::new("riscv64-unknown-elf-gcc")
        .args(["-o"])
        .arg(&ref_exe)
        .arg(source_path)
        .output()
        .map_err(|e| TestError::Reference(format!("Failed to run riscv-gcc: {}", e)))?;

    if !ref_compile.status.success() {
        return Err(TestError::Reference(format!(
            "GCC reference compilation failed:\nstdout: {}\nstderr: {}",
            String::from_utf8_lossy(&ref_compile.stdout),
            String::from_utf8_lossy(&ref_compile.stderr)
        )).into());
    }

    let ref_output = Command::new("spike")
        .args(["pk"])
        .arg(&ref_exe)
        .output()
        .map_err(|e| TestError::Reference(format!("Failed to run spike (reference): {}", e)))?;

    let expected = ref_output.status.code().unwrap_or(-1);

    // ============================================================
    // Test implementation: yasysyc -> GCC (assemble) -> run
    // ============================================================
    let asm_path = temp_dir.path().join("test.S");
    let test_exe = temp_dir.path().join("test");

    // Step 1: Compile SysY to RISC-V assembly with yasysyc
    let compiler = get_compiler_path();
    let compile_status = Command::new(&compiler)
        .args(["--riscv", "-o"])
        .arg(&asm_path)
        .arg(source_path)
        .output()
        .map_err(|e| TestError::Compile(format!("Failed to run yasysyc: {}", e)))?;

    if !compile_status.status.success() {
        return Err(TestError::Compile(format!(
            "yasysyc compilation failed:\nstdout: {}\nstderr: {}",
            String::from_utf8_lossy(&compile_status.stdout),
            String::from_utf8_lossy(&compile_status.stderr)
        )).into());
    }

    // Step 2: Assemble and link with RISC-V GCC
    let assemble_status = Command::new("riscv64-unknown-elf-gcc")
        .args(["-o"])
        .arg(&test_exe)
        .arg(&asm_path)
        .output()
        .map_err(|e| TestError::Assemble(format!("Failed to run riscv-gcc: {}", e)))?;

    if !assemble_status.status.success() {
        let asm_content = fs::read_to_string(&asm_path).unwrap_or_default();
        return Err(TestError::Assemble(format!(
            "riscv-gcc assembly failed:\nstdout: {}\nstderr: {}\n\nGenerated assembly:\n{}",
            String::from_utf8_lossy(&assemble_status.stdout),
            String::from_utf8_lossy(&assemble_status.stderr),
            asm_content
        )).into());
    }

    // Step 3: Run with spike pk
    let run_output = Command::new("spike")
        .args(["pk"])
        .arg(&test_exe)
        .output()
        .map_err(|e| TestError::Run(format!("Failed to run spike: {}", e)))?;

    let actual = run_output.status.code().unwrap_or(-1);

    // ============================================================
    // Compare results
    // ============================================================
    if actual != expected {
        let asm_content = fs::read_to_string(&asm_path).unwrap_or_default();
        return Err(TestError::Mismatch {
            expected,
            actual,
            asm: asm_content,
        }.into());
    }

    Ok(())
}

datatest_stable::harness!(run_test, "tests/e2e/cases", r"\.c$");
