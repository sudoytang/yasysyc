use super::*;

use koopa::ir::{BasicBlock, FunctionData, Program, Type, Value, dfg::DataFlowGraph};
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
        let dfg = func.dfg_mut();
        let value = self.num.emit(dfg);
        let ret_stmt = dfg.new_value().ret(Some(value));
        func.layout_mut().bb_mut(bb).insts_mut().push_key_back(ret_stmt).unwrap();
    }
}

impl Number {
    pub fn emit(&self, dfg: &mut DataFlowGraph) -> Value {
        let value = dfg.new_value().integer(self.value);
        value
    }
}