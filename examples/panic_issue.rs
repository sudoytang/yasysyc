//! Minimal reproduction case for koopa panic issue.
//!
//! Run with: cargo run --example panic_issue
//!
//! This demonstrates that `FunctionData::new()` creates an object that
//! will panic when used, because `dfg.globals` is an uninitialized Weak reference.
//!
//! The correct usage (as shown in official docs) is:
//!   1. program.new_func(FunctionData::new(...))  // Add to Program first
//!   2. program.func_mut(handle)                  // Then get reference through Program
//!
//! But the API allows users to create standalone FunctionData and operate on it,
//! which leads to a confusing runtime panic instead of a compile-time error.

use koopa::ir::builder_traits::*;
use koopa::ir::{FunctionData, Program, Type};

fn main() {
    // Uncomment to verify correct usage works:
    // correct_usage();

    // This is what user might naturally write (WRONG):
    // This will panic - the issue we're reporting
    wrong_usage();
}

/// WRONG: Create standalone FunctionData and operate on it
/// This will panic because dfg.globals.upgrade() returns None
fn wrong_usage() {
    let mut program = Program::new();

    // User creates FunctionData independently
    let mut func = FunctionData::new("@main".into(), vec![], Type::get_i32());

    // User tries to add basic block and instructions (seems reasonable)
    let dfg = func.dfg_mut();
    let entry = dfg.new_bb().basic_block(Some("%entry".into()));

    // User adds basic block to layout
    func.layout_mut().bbs_mut().push_key_back(entry).unwrap();

    // User creates return instruction - PANIC HERE!
    // Because dfg.globals is a Weak ref pointing to nothing
    let dfg = func.dfg_mut();
    let zero = dfg.new_value().integer(0);
    let ret = dfg.new_value().ret(Some(zero)); // <-- PANIC: unwrap() on None

    func.layout_mut().bb_mut(entry).insts_mut().push_key_back(ret).unwrap();

    // User expects to add the completed function to program
    program.new_func(func);
}

/// CORRECT: Add FunctionData to Program first, then operate through Program
#[allow(dead_code)]
fn correct_usage() {
    let mut program = Program::new();

    // Step 1: Create and ADD to Program first
    let main = program.new_func(FunctionData::new(
        "@main".into(),
        vec![],
        Type::get_i32(),
    ));

    // Step 2: Get reference THROUGH Program
    let func = program.func_mut(main);

    // Now operations work correctly
    let entry = func.dfg_mut().new_bb().basic_block(Some("%entry".into()));
    func.layout_mut().bbs_mut().push_key_back(entry).unwrap();

    let dfg = func.dfg_mut();
    let zero = dfg.new_value().integer(0);
    let ret = dfg.new_value().ret(Some(zero)); // Works fine!

    func.layout_mut().bb_mut(entry).insts_mut().push_key_back(ret).unwrap();

    println!("Correct usage works!");
}
