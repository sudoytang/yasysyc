pub mod asm;
pub mod regalloc;

use asm::AsmLine;
use asm::Directive;
use asm::Instruction;
use asm::Reg;
use asm::Section;
use koopa::ir::*;
use regalloc::{Location, RegisterAllocator, StackAllocator};

pub struct AsmGenerator<A: RegisterAllocator> {
    output: Vec<AsmLine>,
    allocator: A,
}

impl AsmGenerator<StackAllocator> {
    /// Create a new AsmGenerator with the default StackAllocator
    pub fn new() -> Self {
        Self::with_allocator(StackAllocator::new())
    }

    /// Convenience method to generate assembly using the default allocator
    pub fn generate(program: &Program) -> String {
        let mut generator = Self::new();
        generator.visit_program(program);
        generator.to_string()
    }
}

impl Default for AsmGenerator<StackAllocator> {
    fn default() -> Self {
        Self::new()
    }
}

impl<A: RegisterAllocator> AsmGenerator<A> {
    /// Create a new AsmGenerator with a custom allocator
    pub fn with_allocator(allocator: A) -> Self {
        Self {
            output: Vec::new(),
            allocator,
        }
    }

    /// Generate assembly using a custom allocator
    pub fn generate_with_allocator(program: &Program, allocator: A) -> String {
        let mut generator = Self::with_allocator(allocator);
        generator.visit_program(program);
        generator.to_string()
    }

    pub fn to_string(&self) -> String {
        self.output
            .iter()
            .map(|line| line.to_string())
            .collect::<Vec<_>>()
            .join("\n")
    }

    /// Emit an instruction
    fn emit(&mut self, inst: Instruction) {
        self.output.push(AsmLine::Instruction(inst));
    }

    /// Store a register value to the location allocated for a given IR value
    fn store_value(&mut self, value: Value, reg: Reg) {
        match self.allocator.alloc(value) {
            Location::Stack(offset) => {
                self.emit(Instruction::Sw {
                    rs: reg,
                    offset,
                    base: Reg::Sp,
                });
            }
            Location::Register(dest) => {
                if dest != reg {
                    self.emit(Instruction::Mv { rd: dest, rs: reg });
                }
            }
            Location::Immediate(_) => {
                panic!("Cannot store to an immediate location");
            }
        }
    }

    /// Load a value into a register, emitting necessary instructions
    fn load_value(&mut self, func: &FunctionData, value: Value, dest_reg: Reg) -> Reg {
        let value_data = func.dfg().value(value);

        use koopa::ir::ValueKind;
        match value_data.kind() {
            ValueKind::Integer(int_val) => {
                let imm = int_val.value();

                if imm == 0 {
                    return Reg::Zero;
                }

                self.emit(Instruction::Li { reg: dest_reg, imm });
                dest_reg
            }
            _ => {
                match self.allocator.locate(value) {
                    Some(Location::Stack(offset)) => {
                        self.emit(Instruction::Lw {
                            rd: dest_reg,
                            offset,
                            base: Reg::Sp,
                        });
                        dest_reg
                    }
                    Some(Location::Register(reg)) => {
                        if reg != dest_reg {
                            self.emit(Instruction::Mv {
                                rd: dest_reg,
                                rs: reg,
                            });
                            dest_reg
                        } else {
                            reg
                        }
                    }
                    Some(Location::Immediate(imm)) => {
                        if imm == 0 {
                            Reg::Zero
                        } else {
                            self.emit(Instruction::Li { reg: dest_reg, imm });
                            dest_reg
                        }
                    }
                    None => panic!("Value not found in allocator"),
                }
            }
        }
    }

    /// Load a value for return, with stack pointer already restored
    fn load_value_for_return(
        &mut self,
        func: &FunctionData,
        value: Value,
        stack_size: i32,
    ) -> Reg {
        let value_data = func.dfg().value(value);

        use koopa::ir::ValueKind;
        match value_data.kind() {
            ValueKind::Integer(int_val) => {
                let imm = int_val.value();

                if imm == 0 {
                    return Reg::Zero;
                }

                self.emit(Instruction::Li { reg: Reg::T0, imm });
                Reg::T0
            }
            _ => {
                match self.allocator.locate(value) {
                    Some(Location::Stack(offset)) => {
                        // After epilogue (sp += stack_size), adjust offset
                        let adjusted_offset = offset - stack_size;
                        self.emit(Instruction::Lw {
                            rd: Reg::T0,
                            offset: adjusted_offset,
                            base: Reg::Sp,
                        });
                        Reg::T0
                    }
                    Some(Location::Register(reg)) => reg,
                    Some(Location::Immediate(imm)) => {
                        if imm == 0 {
                            Reg::Zero
                        } else {
                            self.emit(Instruction::Li { reg: Reg::T0, imm });
                            Reg::T0
                        }
                    }
                    None => panic!("Value not found in allocator"),
                }
            }
        }
    }

    pub fn visit_program(&mut self, program: &Program) {
        for &func in program.func_layout() {
            let func = program.func(func);
            self.visit_func(func);
        }
    }

    pub fn visit_func(&mut self, func: &FunctionData) {
        // Strip @ prefix from function name for assembly
        let func_name = func.name().strip_prefix('@').unwrap_or(func.name());

        self.output
            .push(AsmLine::Directive(Directive::Section(Section::Text)));
        self.output
            .push(AsmLine::Directive(Directive::Global(func_name.to_string())));
        self.output.push(AsmLine::Label(func_name.to_string()));

        // Reset and analyze for this function
        self.allocator.reset();
        self.allocator.analyze(func);

        let stack_size = self.allocator.stack_size();

        // Emit prologue: allocate stack frame
        if stack_size > 0 {
            self.emit(Instruction::Addi {
                rd: Reg::Sp,
                rs: Reg::Sp,
                imm: -stack_size,
            });
        }

        // Generate code for all instructions
        for (_bb, node) in func.layout().bbs() {
            for inst in node.insts().keys() {
                self.visit_instruction(func, inst, stack_size);
            }
        }
    }

    pub fn visit_instruction(&mut self, func: &FunctionData, inst: &Value, stack_size: i32) {
        let value_data = func.dfg().value(*inst);

        use koopa::ir::ValueKind;
        match value_data.kind() {
            ValueKind::Binary(binary) => {
                let lhs = binary.lhs();
                let rhs = binary.rhs();

                use koopa::ir::BinaryOp;
                match binary.op() {
                    BinaryOp::Add => {
                        let lhs_reg = self.load_value(func, lhs, Reg::T0);
                        let rhs_reg = self.load_value(func, rhs, Reg::T1);
                        self.emit(Instruction::Add {
                            rd: Reg::T2,
                            rs1: lhs_reg,
                            rs2: rhs_reg,
                        });
                        self.store_value(*inst, Reg::T2);
                    }
                    BinaryOp::Sub => {
                        let lhs_reg = self.load_value(func, lhs, Reg::T0);
                        let rhs_reg = self.load_value(func, rhs, Reg::T1);
                        self.emit(Instruction::Sub {
                            rd: Reg::T2,
                            rs1: lhs_reg,
                            rs2: rhs_reg,
                        });
                        self.store_value(*inst, Reg::T2);
                    }
                    BinaryOp::Mul => {
                        let lhs_reg = self.load_value(func, lhs, Reg::T0);
                        let rhs_reg = self.load_value(func, rhs, Reg::T1);
                        self.emit(Instruction::Mul {
                            rd: Reg::T2,
                            rs1: lhs_reg,
                            rs2: rhs_reg,
                        });
                        self.store_value(*inst, Reg::T2);
                    }
                    BinaryOp::Div => {
                        let lhs_reg = self.load_value(func, lhs, Reg::T0);
                        let rhs_reg = self.load_value(func, rhs, Reg::T1);
                        self.emit(Instruction::Div {
                            rd: Reg::T2,
                            rs1: lhs_reg,
                            rs2: rhs_reg,
                        });
                        self.store_value(*inst, Reg::T2);
                    }
                    BinaryOp::Mod => {
                        let lhs_reg = self.load_value(func, lhs, Reg::T0);
                        let rhs_reg = self.load_value(func, rhs, Reg::T1);
                        self.emit(Instruction::Rem {
                            rd: Reg::T2,
                            rs1: lhs_reg,
                            rs2: rhs_reg,
                        });
                        self.store_value(*inst, Reg::T2);
                    }
                    BinaryOp::Eq => {
                        let lhs_reg = self.load_value(func, lhs, Reg::T0);
                        let rhs_reg = self.load_value(func, rhs, Reg::T1);
                        // x == y => sub t2, t0, t1; seqz t2, t2
                        self.emit(Instruction::Sub {
                            rd: Reg::T2,
                            rs1: lhs_reg,
                            rs2: rhs_reg,
                        });
                        self.emit(Instruction::Seqz {
                            rd: Reg::T2,
                            rs: Reg::T2,
                        });
                        self.store_value(*inst, Reg::T2);
                    }
                    BinaryOp::NotEq => {
                        let lhs_reg = self.load_value(func, lhs, Reg::T0);
                        let rhs_reg = self.load_value(func, rhs, Reg::T1);
                        // x != y => sub t2, t0, t1; snez t2, t2
                        self.emit(Instruction::Sub {
                            rd: Reg::T2,
                            rs1: lhs_reg,
                            rs2: rhs_reg,
                        });
                        self.emit(Instruction::Snez {
                            rd: Reg::T2,
                            rs: Reg::T2,
                        });
                        self.store_value(*inst, Reg::T2);
                    }
                    _ => unimplemented!("Unsupported binary operator: {:?}", binary.op()),
                }
            }
            ValueKind::Return(ret_val) => {
                // Emit epilogue before return
                if stack_size > 0 {
                    self.emit(Instruction::Addi {
                        rd: Reg::Sp,
                        rs: Reg::Sp,
                        imm: stack_size,
                    });
                }

                // If there is a return value
                if let Some(val_handle) = ret_val.value() {
                    // Load return value into t0, then move to a0
                    let val_reg = self.load_value_for_return(func, val_handle, stack_size);

                    // Move to a0 if not already there
                    if !matches!(val_reg, Reg::A0) {
                        self.emit(Instruction::Mv {
                            rd: Reg::A0,
                            rs: val_reg,
                        });
                    }
                }

                // ret instruction
                self.emit(Instruction::Ret);
            }
            _ => unimplemented!("Unsupported instruction: {:?}", value_data.kind()),
        }
    }
}
