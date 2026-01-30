use super::*;

use std::collections::HashMap;

use koopa::ir::{BasicBlock, FunctionData, Program, Type, Value};
use koopa::ir::builder_traits::*;



pub struct EmitContext {
    const_table: HashMap<Ident, ConstExpr>,
    var_table: HashMap<Ident, Value>,
}

impl EmitContext {
    pub fn new() -> Self {
        Self {
            const_table: HashMap::new(),
            var_table: HashMap::new(),
        }
    }
}

impl Default for EmitContext {
    fn default() -> Self {
        Self::new()
    }
}

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
        let mut context = EmitContext::new();
        let func =program.new_func(func);
        let func = program.func_mut(func);
        self.block.emit(func, &mut context);
    }
}

impl Block {
    pub fn emit(&self, func: &mut FunctionData, context: &mut EmitContext) {
        let entry = func.dfg_mut().new_bb().basic_block(Some("%entry".into()));
        func.layout_mut().bbs_mut().push_key_back(entry).unwrap();
        for item in &self.items {
            item.emit(func, entry, context);
        }
    }
}

impl BlockItem {
    pub fn emit(&self, func: &mut FunctionData, bb: BasicBlock, context: &mut EmitContext) {
        match self {
            Self::Stmt(stmt) => stmt.emit(func, bb, context),
            Self::Decl(decl) => decl.emit(func, bb, context),
        }
    }
}

impl Decl {
    pub fn emit(&self, func: &mut FunctionData, bb: BasicBlock, context: &mut EmitContext) {
        match self {
            Self::Const(const_decl) => const_decl.emit(context),
            Self::Var(var_decl) => var_decl.emit(func, bb, context),
        }
    }
}

impl ConstDecl {
    pub fn emit(&self, context: &mut EmitContext) {
        for def in &self.defs {
            context.const_table.insert(def.id.clone(), def.init.const_expr.clone());
        }
    }
}

impl VarDecl {
    pub fn emit(&self, func: &mut FunctionData, bb: BasicBlock, context: &mut EmitContext) {
        for def in &self.defs {
            // alloc i32
            let alloc = func.dfg_mut().new_value().alloc(Type::get_i32());
            func.dfg_mut().set_value_name(alloc, Some(def.id.emit()));
            func.layout_mut().bb_mut(bb).insts_mut().push_key_back(alloc).unwrap();

            // store to var_table
            context.var_table.insert(def.id.clone(), alloc);

            // if has init, generate store
            if let Some(init) = &def.init {
                let value = init.expr.emit(func, bb, context);
                let store = func.dfg_mut().new_value().store(value, alloc);
                func.layout_mut().bb_mut(bb).insts_mut().push_key_back(store).unwrap();
            }
        }
    }
}

impl Stmt {
    pub fn emit(&self, func: &mut FunctionData, bb: BasicBlock, context: &mut EmitContext) {
        match self {
            Self::Return(return_stmt) => return_stmt.emit(func, bb, context),
            Self::Assign(assign_stmt) => assign_stmt.emit(func, bb, context),
        }
    }
}

impl ReturnStmt {
    pub fn emit(&self, func: &mut FunctionData, bb: BasicBlock, context: &mut EmitContext) {
        let value = self.expr.emit(func, bb, context);
        let ret_stmt = func.dfg_mut().new_value().ret(Some(value));
        func.layout_mut().bb_mut(bb).insts_mut().push_key_back(ret_stmt).unwrap();
    }
}

impl AssignStmt {
    pub fn emit(&self, func: &mut FunctionData, bb: BasicBlock, context: &mut EmitContext) {
        let addr = *context.var_table.get(&self.lval.ident).unwrap();
        let value = self.expr.emit(func, bb, context);
        let store = func.dfg_mut().new_value().store(value, addr);
        func.layout_mut().bb_mut(bb).insts_mut().push_key_back(store).unwrap();
    }
}

impl Expr {
    pub fn emit(&self, func: &mut FunctionData, bb: BasicBlock, context: &EmitContext) -> Value {
        match self {
            Self::Number(number) => func.dfg_mut().new_value().integer(*number),
            Self::Unary(unary_op, expr) => {
                let value = expr.emit(func, bb, context);

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
            Self::Binary(lhs, op, rhs) => {
                let lhs_val = lhs.emit(func, bb, context);
                let rhs_val = rhs.emit(func, bb, context);

                // Special handling for logical operators (Koopa IR only has bitwise Or/And)
                match op {
                    BinaryOp::Or => {
                        // a || b => (a | b) != 0
                        let or_val = func.dfg_mut().new_value().binary(koopa::ir::BinaryOp::Or, lhs_val, rhs_val);
                        func.layout_mut().bb_mut(bb).insts_mut().push_key_back(or_val).unwrap();
                        let zero = func.dfg_mut().new_value().integer(0);
                        let result = func.dfg_mut().new_value().binary(koopa::ir::BinaryOp::NotEq, or_val, zero);
                        func.layout_mut().bb_mut(bb).insts_mut().push_key_back(result).unwrap();
                        result
                    }
                    BinaryOp::And => {
                        // a && b => (a != 0) & (b != 0)
                        let zero = func.dfg_mut().new_value().integer(0);
                        let lhs_bool = func.dfg_mut().new_value().binary(koopa::ir::BinaryOp::NotEq, lhs_val, zero);
                        func.layout_mut().bb_mut(bb).insts_mut().push_key_back(lhs_bool).unwrap();
                        let zero2 = func.dfg_mut().new_value().integer(0);
                        let rhs_bool = func.dfg_mut().new_value().binary(koopa::ir::BinaryOp::NotEq, rhs_val, zero2);
                        func.layout_mut().bb_mut(bb).insts_mut().push_key_back(rhs_bool).unwrap();
                        let result = func.dfg_mut().new_value().binary(koopa::ir::BinaryOp::And, lhs_bool, rhs_bool);
                        func.layout_mut().bb_mut(bb).insts_mut().push_key_back(result).unwrap();
                        result
                    }
                    _ => {
                        let ir_op = op.emit();
                        let value = func.dfg_mut().new_value().binary(ir_op, lhs_val, rhs_val);
                        func.layout_mut().bb_mut(bb).insts_mut().push_key_back(value).unwrap();
                        value
                    }
                }
            }
            Self::LVal(lval) => {
                // first check const_table
                if let Some(expr) = context.const_table.get(&lval.ident) {
                    return expr.expr.emit(func, bb, context);
                }
                // then check var_table
                let addr = *context.var_table.get(&lval.ident).unwrap();
                let load = func.dfg_mut().new_value().load(addr);
                func.layout_mut().bb_mut(bb).insts_mut().push_key_back(load).unwrap();
                load
            }
        }
    }
}

impl BinaryOp {
    pub fn emit(&self) -> koopa::ir::BinaryOp {
        match self {
            Self::Add => koopa::ir::BinaryOp::Add,
            Self::Sub => koopa::ir::BinaryOp::Sub,
            Self::Mul => koopa::ir::BinaryOp::Mul,
            Self::Div => koopa::ir::BinaryOp::Div,
            Self::Mod => koopa::ir::BinaryOp::Mod,
            Self::Or => koopa::ir::BinaryOp::Or,
            Self::And => koopa::ir::BinaryOp::And,
            Self::Eq => koopa::ir::BinaryOp::Eq,
            Self::Ne => koopa::ir::BinaryOp::NotEq,
            Self::Lt => koopa::ir::BinaryOp::Lt,
            Self::Gt => koopa::ir::BinaryOp::Gt,
            Self::Le => koopa::ir::BinaryOp::Le,
            Self::Ge => koopa::ir::BinaryOp::Ge,
        }
    }
}