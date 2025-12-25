pub mod asm;

use asm::AsmLine;
use asm::Directive;
use asm::Section;
use koopa::ir::*;

pub struct AsmGenerator {
    output: Vec<AsmLine>,
}

impl AsmGenerator {
    pub fn new() -> Self {
        Self {
            output: Vec::new(),
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
            ValueKind::Return(ret_val) => {
                // If there is a return value
                if let Some(val_handle) = ret_val.value() {
                    // Query the return value data
                    let ret_val_data = func.dfg().value(val_handle);

                    // For lv1, return value can only be integer constant
                    if let ValueKind::Integer(int_val) = ret_val_data.kind() {
                        // li a0, constant_value
                        self.output.push(AsmLine::Instruction(
                            asm::Instruction::Li {
                                reg: asm::Reg::A0,
                                imm: int_val.value(),
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
}
