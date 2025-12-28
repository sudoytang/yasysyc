use super::asm::Reg;
use koopa::ir::{FunctionData, Value, ValueKind};
use std::collections::HashMap;

/// Location of a value - either in a register, on the stack, or an immediate
#[derive(Debug, Clone, Copy)]
pub enum Location {
    /// Value is in a register
    Register(Reg),
    /// Value is on the stack at the given offset from sp
    Stack(i32),
    /// Value is an immediate constant
    Immediate(i32),
}

/// Trait for register allocation strategies
pub trait RegisterAllocator {
    /// Analyze a function to prepare for allocation (e.g., compute live ranges)
    fn analyze(&mut self, func: &FunctionData);

    /// Allocate storage for a value, returns the location
    fn alloc(&mut self, value: Value) -> Location;

    /// Query where a value is currently stored
    fn locate(&self, value: Value) -> Option<Location>;

    /// Get the total stack frame size needed (for prologue/epilogue)
    fn stack_size(&self) -> i32;

    /// Reset state for a new function
    fn reset(&mut self);
}

/// Stack-based allocator: all values go to the stack
/// Uses only t0, t1, t2 as scratch registers for computation
pub struct StackAllocator {
    /// Map from Value to stack offset
    value_stack_offset: HashMap<Value, i32>,
    /// Current stack frame size (before alignment)
    current_offset: i32,
    /// Aligned stack frame size
    aligned_stack_size: i32,
}

impl StackAllocator {
    pub fn new() -> Self {
        Self {
            value_stack_offset: HashMap::new(),
            current_offset: 0,
            aligned_stack_size: 0,
        }
    }
}

impl Default for StackAllocator {
    fn default() -> Self {
        Self::new()
    }
}

impl RegisterAllocator for StackAllocator {
    fn analyze(&mut self, func: &FunctionData) {
        // First pass: count all values that need stack slots
        let mut stack_size = 0;
        for (_bb, node) in func.layout().bbs() {
            for inst in node.insts().keys() {
                let value_data = func.dfg().value(*inst);
                // Instructions that produce a value need a stack slot
                // (Return doesn't produce a value)
                if !matches!(value_data.kind(), ValueKind::Return(_)) {
                    stack_size += 4;
                }
            }
        }

        // Align to 16 bytes (RISC-V ABI requirement)
        self.aligned_stack_size = (stack_size + 15) & !15;
    }

    fn alloc(&mut self, value: Value) -> Location {
        let offset = self.current_offset;
        self.value_stack_offset.insert(value, offset);
        self.current_offset += 4;
        Location::Stack(offset)
    }

    fn locate(&self, value: Value) -> Option<Location> {
        self.value_stack_offset
            .get(&value)
            .map(|&offset| Location::Stack(offset))
    }

    fn stack_size(&self) -> i32 {
        self.aligned_stack_size
    }

    fn reset(&mut self) {
        self.value_stack_offset.clear();
        self.current_offset = 0;
        self.aligned_stack_size = 0;
    }
}
