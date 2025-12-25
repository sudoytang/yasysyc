# Confusing panic when using FunctionData without adding to Program first

## Description

When a user creates a `FunctionData` using `FunctionData::new()` and tries to operate on it directly (without first adding it to a `Program`), a confusing panic occurs:

```
thread 'main' panicked at .cargo/registry/src/index.crates.io-1949cf8c6b5b557f/koopa-0.0.9/src/ir/builder.rs:445:8:
called `Option::unwrap()` on a `None` value
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/ded5c06cf21d2b93bffd5d884aa6e96934ee4234/library/std/src/panicking.rs:698:5
   1: core::panicking::panic_fmt
             at /rustc/ded5c06cf21d2b93bffd5d884aa6e96934ee4234/library/core/src/panicking.rs:80:14
   2: core::panicking::panic
             at /rustc/ded5c06cf21d2b93bffd5d884aa6e96934ee4234/library/core/src/panicking.rs:150:5
   3: core::option::unwrap_failed
             at /rustc/ded5c06cf21d2b93bffd5d884aa6e96934ee4234/library/core/src/option.rs:2174:5
   4: core::option::Option<T>::unwrap
             at .rustup/toolchains/stable-aarch64-apple-darwin/lib/rustlib/src/rust/library/core/src/option.rs:1015:21
   5: <T as koopa::ir::builder::EntityInfoQuerier>::value_type
             at .cargo/registry/src/index.crates.io-1949cf8c6b5b557f/koopa-0.0.9/src/ir/builder.rs:445:8
   6: koopa::ir::builder::LocalInstBuilder::ret::{{closure}}
             at .cargo/registry/src/index.crates.io-1949cf8c6b5b557f/koopa-0.0.9/src/ir/builder.rs:330:34
   7: core::option::Option<T>::is_none_or
             at .rustup/toolchains/stable-aarch64-apple-darwin/lib/rustlib/src/rust/library/core/src/option.rs:712:24
   8: koopa::ir::builder::LocalInstBuilder::ret
             at .cargo/registry/src/index.crates.io-1949cf8c6b5b557f/koopa-0.0.9/src/ir/builder.rs:330:13
```

The root cause seems to be that `FunctionData::new()` creates an object with `dfg.globals` as an uninitialized `Weak` reference.

When `ret()` (or similar methods) tries to query value types via `EntityInfoQuerier::value_type()`, it calls `globals.upgrade().unwrap()` which panics.

```rust
  fn value_type(&self, value: Value) -> Type {
    self
      .dfg()
      .globals
      .upgrade()
      .unwrap()
      .borrow()
      .get(&value)
      .or_else(|| self.dfg().values().get(&value))
      .expect("value does not exist")
      .ty()
      .clone()
  }
```
(Code of `<T as koopa::ir::builder::EntityInfoQuerier>::value_type`)

## Reproduction

```rust
use koopa::ir::builder_traits::*;
use koopa::ir::{FunctionData, Program, Type};

fn main() {
    let mut program = Program::new();

    // User creates FunctionData independently (seems reasonable)
    let mut func = FunctionData::new("@main".into(), vec![], Type::get_i32());

    // Add basic block
    let entry = func.dfg_mut().new_bb().basic_block(Some("%entry".into()));
    func.layout_mut().bbs_mut().push_key_back(entry).unwrap();

    // Try to create ret instruction - PANIC!
    let dfg = func.dfg_mut();
    let zero = dfg.new_value().integer(0);
    let ret = dfg.new_value().ret(Some(zero)); // panics here

    func.layout_mut().bb_mut(entry).insts_mut().push_key_back(ret).unwrap();
    program.new_func(func);
}
```

## Expected Behavior

Either:
1. **Compile-time error**: The API should prevent this usage pattern, OR
2. **Clear runtime error**: A meaningful error message like `"FunctionData must be added to Program before creating instructions"`

## Actual Behavior

A confusing panic with no indication of what the user did wrong:
```
called `Option::unwrap()` on a `None` value
```

## Correct Usage (for reference)

The correct way is to add `FunctionData` to `Program` first, then operate through `Program`:

```rust
let main = program.new_func(FunctionData::new("@main".into(), vec![], Type::get_i32()));
let func = program.func_mut(main);  // Get reference through Program
// Now operations work correctly
```

## Suggested Fixes

1. **Better error message**: Replace `.unwrap()` with `.expect("FunctionData must be added to Program before use")`

2. **Documentation**: Add a warning to `FunctionData::new()` docs explaining that the returned `FunctionData` must be added to a `Program` via `new_func()` before any operations on it

3. **API redesign** (optional): Change `Program::new_func` to accept construction parameters directly instead of a `FunctionData`:
   ```rust
   // Instead of:
   program.new_func(FunctionData::new("@main".into(), vec![], Type::get_i32()))

   // Could be:
   program.new_func("@main".into(), vec![], Type::get_i32())
   ```
   This way `FunctionData` would only be created internally by `Program`, preventing users from operating on uninitialized instances

## Environment

- koopa version: 0.0.9
