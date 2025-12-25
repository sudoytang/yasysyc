use super::*;

use koopa::ir::{BasicBlock, FunctionData, Program, Type, Value};
use koopa::ir::builder_traits::*;
impl Ident {
    pub fn emit(&self) -> String {
        format!("@{}", self.value)
    }
}

impl FuncType {
    pub fn emit(&self) -> Type {
        match self {
            Self::Int => Type::get_i32(),
        }
    }
}





impl CompUnit {
    pub fn emit(&self) -> Program {
        let mut prgm = Program::new();
        self.func_def.emit(&mut prgm);
        prgm
    }
}




impl FuncDef {
    pub fn emit(&self, program: &mut Program) {
        let func = FunctionData::new(
            self.id.emit(),
            vec![],
            self.func_type.emit(),
        );
        let func =program.new_func(func);
        let func = program.func_mut(func);
        self.block.emit(func);
    }
}

impl Block {
    pub fn emit(&self, func: &mut FunctionData) {
        let entry = func.dfg_mut().new_bb().basic_block(Some("%entry".into()));
        func.layout_mut().bbs_mut().push_key_back(entry).unwrap();
        self.stmt.emit(func, entry);
    }
}

impl Stmt {
    pub fn emit(&self, func: &mut FunctionData, bb: BasicBlock) {
        match self {
            Self::Return(return_stmt) => return_stmt.emit(func, bb),
        }
    }
}

impl ReturnStmt {
    pub fn emit(&self, func: &mut FunctionData, bb: BasicBlock) {
        let value = self.expr.emit(func, bb);
        let ret_stmt = func.dfg_mut().new_value().ret(Some(value));
        func.layout_mut().bb_mut(bb).insts_mut().push_key_back(ret_stmt).unwrap();
    }
}

impl Expr {
    pub fn emit(&self, func: &mut FunctionData, bb: BasicBlock) -> Value {
        match self {
            Self::Number(number) => func.dfg_mut().new_value().integer(*number),
            Self::Unary(unary_op, expr) => {
                let value = expr.emit(func, bb);

                match unary_op {
                    // +x => x
                    UnaryOp::Plus => value,

                    // -x => 0 - x
                    UnaryOp::Minus => {
                        let zero = func.dfg_mut().new_value().integer(0);
                        let sub = func.dfg_mut().new_value().binary(koopa::ir::BinaryOp::Sub, zero, value);
                        func.layout_mut().bb_mut(bb).insts_mut().push_key_back(sub).unwrap();
                        sub
                    }

                    // !x => x == 0
                    UnaryOp::Not => {
                        let zero = func.dfg_mut().new_value().integer(0);
                        let eq = func.dfg_mut().new_value().binary(koopa::ir::BinaryOp::Eq, value, zero);
                        func.layout_mut().bb_mut(bb).insts_mut().push_key_back(eq).unwrap();
                        eq
                    }
                }
            }
        }
    }
}