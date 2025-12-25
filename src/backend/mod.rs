pub mod asm;

use asm::AsmLine;
use asm::Directive;
use asm::Section;
use koopa::ir::*;
use std::collections::HashMap;

pub struct AsmGenerator {
    output: Vec<AsmLine>,
    // Map from Koopa IR Value to register
    value_reg_map: HashMap<Value, asm::Reg>,
    // Next available temporary register index
    next_temp_reg: usize,
}

impl AsmGenerator {
    pub fn new() -> Self {
        Self {
            output: Vec::new(),
            value_reg_map: HashMap::new(),
            next_temp_reg: 0,
        }
    }

    pub fn generate(program: &Program) -> String {
        let mut generator = Self::new();
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

    // Allocate a temporary register for a value
    fn alloc_temp_reg(&mut self, value: Value) -> asm::Reg {
        let reg = match self.next_temp_reg {
            0 => asm::Reg::T0,
            1 => asm::Reg::T1,
            2 => asm::Reg::T2,
            3 => asm::Reg::T3,
            4 => asm::Reg::T4,
            5 => asm::Reg::T5,
            6 => asm::Reg::T6,
            _ => panic!("Out of temporary registers"),
        };
        self.next_temp_reg += 1;
        self.value_reg_map.insert(value, reg);
        reg
    }

    // Get the register for a value (or allocate one)
    fn get_or_alloc_reg(&mut self, value: Value) -> asm::Reg {
        if let Some(&reg) = self.value_reg_map.get(&value) {
            reg
        } else {
            self.alloc_temp_reg(value)
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

        self.output.push(AsmLine::Directive(Directive::Section(Section::Text)));
        self.output.push(AsmLine::Directive(Directive::Global(func_name.to_string())));
        self.output.push(AsmLine::Label(func_name.to_string()));

        // Reset register allocation for each function
        self.value_reg_map.clear();
        self.next_temp_reg = 0;

        for (_bb, node) in func.layout().bbs() {
            for inst in node.insts().keys() {
                self.visit_instruction(func, inst);
            }
        }
    }

    pub fn visit_instruction(&mut self, func: &FunctionData, inst: &Value) {
        // Get instruction data
        let value_data = func.dfg().value(*inst);

        use koopa::ir::ValueKind;
        match value_data.kind() {
            ValueKind::Binary(binary) => {
                // Get operands
                let lhs = binary.lhs();
                let rhs = binary.rhs();

                // Generate instruction based on operator
                use koopa::ir::BinaryOp;
                match binary.op() {
                    BinaryOp::Sub => {
                        // Check if lhs is 0 (unary negation: -x = 0 - x)
                        let lhs_data = func.dfg().value(lhs);
                        let is_lhs_zero = matches!(lhs_data.kind(),
                            ValueKind::Integer(i) if i.value() == 0);

                        if is_lhs_zero {
                            // Unary negation: sub rd, zero, rs
                            let rs = self.load_value(func, rhs);

                            // Can't modify zero register, need to allocate if rs is zero
                            if matches!(rs, asm::Reg::Zero) {
                                let rd = self.alloc_temp_reg(*inst);
                                self.output.push(AsmLine::Instruction(
                                    asm::Instruction::Sub { rd, rs1: asm::Reg::Zero, rs2: rs }
                                ));
                            } else {
                                // In-place optimization: reuse rs as rd
                                self.value_reg_map.insert(*inst, rs);
                                self.output.push(AsmLine::Instruction(
                                    asm::Instruction::Sub { rd: rs, rs1: asm::Reg::Zero, rs2: rs }
                                ));
                            }
                        } else {
                            // Binary subtraction: need to allocate new register
                            let lhs_reg = self.load_value(func, lhs);
                            let rhs_reg = self.load_value(func, rhs);
                            let rd = self.alloc_temp_reg(*inst);
                            self.output.push(AsmLine::Instruction(
                                asm::Instruction::Sub { rd, rs1: lhs_reg, rs2: rhs_reg }
                            ));
                        }
                    }
                    BinaryOp::Eq => {
                        // Check if rhs is 0 (comparison with zero: x == 0)
                        let rhs_data = func.dfg().value(rhs);
                        let is_rhs_zero = matches!(rhs_data.kind(),
                            ValueKind::Integer(i) if i.value() == 0);

                        if is_rhs_zero {
                            // x == 0 => seqz rd, rs
                            let rs = self.load_value(func, lhs);

                            // Can't modify zero register, need to allocate if rs is zero
                            if matches!(rs, asm::Reg::Zero) {
                                let rd = self.alloc_temp_reg(*inst);
                                self.output.push(AsmLine::Instruction(
                                    asm::Instruction::Seqz { rd, rs }
                                ));
                            } else {
                                // In-place optimization: reuse rs as rd
                                self.value_reg_map.insert(*inst, rs);
                                self.output.push(AsmLine::Instruction(
                                    asm::Instruction::Seqz { rd: rs, rs }
                                ));
                            }
                        } else {
                            // General case: sub then seqz
                            let lhs_reg = self.load_value(func, lhs);
                            let rhs_reg = self.load_value(func, rhs);
                            let rd = self.alloc_temp_reg(*inst);
                            self.output.push(AsmLine::Instruction(
                                asm::Instruction::Sub { rd, rs1: lhs_reg, rs2: rhs_reg }
                            ));
                            self.output.push(AsmLine::Instruction(
                                asm::Instruction::Seqz { rd, rs: rd }
                            ));
                        }
                    }
                    _ => unimplemented!("Unsupported binary operator: {:?}", binary.op()),
                }
            }
            ValueKind::Return(ret_val) => {
                // If there is a return value
                if let Some(val_handle) = ret_val.value() {
                    // Load return value into a0
                    let val_reg = self.load_value(func, val_handle);

                    // Move to a0 if not already there
                    if !matches!(val_reg, asm::Reg::A0) {
                        self.output.push(AsmLine::Instruction(
                            asm::Instruction::Mv {
                                rd: asm::Reg::A0,
                                rs: val_reg,
                            }
                        ));
                    }
                }
                // ret instruction
                self.output.push(AsmLine::Instruction(asm::Instruction::Ret));
            }
            _ => unimplemented!("Unsupported instruction: {:?}", value_data.kind()),
        }
    }

    // Load a value into a register
    fn load_value(&mut self, func: &FunctionData, value: Value) -> asm::Reg {
        let value_data = func.dfg().value(value);

        use koopa::ir::ValueKind;
        match value_data.kind() {
            ValueKind::Integer(int_val) => {
                let imm = int_val.value();

                // Optimization: use zero register for 0
                if imm == 0 {
                    return asm::Reg::Zero;
                }

                // Constant: load into a new temp register
                let reg = match self.next_temp_reg {
                    0 => asm::Reg::T0,
                    1 => asm::Reg::T1,
                    2 => asm::Reg::T2,
                    3 => asm::Reg::T3,
                    4 => asm::Reg::T4,
                    5 => asm::Reg::T5,
                    6 => asm::Reg::T6,
                    _ => panic!("Out of temporary registers"),
                };
                self.next_temp_reg += 1;

                self.output.push(AsmLine::Instruction(
                    asm::Instruction::Li { reg, imm }
                ));
                reg
            }
            _ => {
                // Already computed value: get its register
                self.get_or_alloc_reg(value)
            }
        }
    }
}
