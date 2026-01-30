pub mod emit;

use std::fmt::{self, Display};

#[derive(Debug)]
pub struct CompUnit {
    pub func_def: FuncDef,
}

impl Display for CompUnit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.func_def)
    }
}

#[derive(Debug)]
pub struct FuncDef {
    pub func_type: FuncType,
    pub id: Ident,
    pub block: Block,
}

impl Display for FuncDef {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}() {}", self.func_type, self.id, self.block)
    }
}

#[non_exhaustive]
#[derive(Debug)]
pub enum FuncType {
    Int,
}

impl Display for FuncType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Int => write!(f, "int"),
        }
    }
}

#[derive(Debug)]
pub struct Block {
    pub items: Vec<BlockItem>,
}

impl Display for Block {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{")?;
        for item in &self.items {
            writeln!(f, "{}", item)?;
        }
        write!(f, "}}")
    }
}



#[non_exhaustive]
#[derive(Debug)]
pub enum Stmt {
    Return(ReturnStmt),
    Assign(AssignStmt),
}

impl Display for Stmt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Return(return_stmt) => write!(f, "{}", return_stmt),
            Self::Assign(assign_stmt) => write!(f, "{}", assign_stmt),
        }
    }
}

#[derive(Debug)]
pub struct ReturnStmt {
    pub expr: Expr,
}

impl Display for ReturnStmt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "return {};", self.expr)
    }
}

#[derive(Debug)]
pub struct AssignStmt {
    pub lval: LVal,
    pub expr: Expr,
}

impl Display for AssignStmt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} = {};", self.lval, self.expr)
    }
}

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
pub struct Ident {
    pub value: String,
}

impl From<String> for Ident {
    fn from(value: String) -> Self {
        Ident { value }
    }
}

impl Display for Ident {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

#[derive(Debug, Clone)]
pub struct LVal {
    pub ident: Ident,
    pub indices: Vec<Expr>,
}

impl Display for LVal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.ident)?;
        for idx in &self.indices {
            write!(f, "[{}]", idx)?;
        }
        Ok(())
    }
}

#[non_exhaustive]
#[derive(Debug)]
pub enum Expr {
    Number(i32),
    Unary(UnaryOp, Box<Expr>),
    Binary(Box<Expr>, BinaryOp, Box<Expr>),
    LVal(LVal),
}

impl Clone for Expr {
    fn clone(&self) -> Self {
        match self {
            Self::Number(number) => Self::Number(*number),
            Self::Unary(unary_op, expr) => Self::Unary(*unary_op, expr.clone()),
            Self::Binary(lhs, op, rhs) => Self::Binary(lhs.clone(), *op, rhs.clone()),
            Self::LVal(lval) => Self::LVal(lval.clone()),
        }
    }
}

impl Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Number(number) => write!(f, "{}", number),
            Self::Unary(unary_op, expr) => write!(f, "{}{}", unary_op, expr),
            Self::Binary(lhs, op, rhs) => write!(f, "({} {} {})", lhs, op, rhs),
            // TODO: we don't know the precedence of binary operations and we are lazy
            // so that a pair of parentheses is added.
            Self::LVal(lval) => write!(f, "{}", lval),
        }
    }
}


#[non_exhaustive]
#[derive(Debug, Clone, Copy)]
pub enum UnaryOp {
    Plus,
    Minus,
    Not,
}

impl Display for UnaryOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Plus => write!(f, "+"),
            Self::Minus => write!(f, "-"),
            Self::Not => write!(f, "!"),
        }
    }
}

#[non_exhaustive]
#[derive(Debug, Clone, Copy)]
pub enum BinaryOp {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Or,
    And,
    Eq,
    Ne,
    Lt,
    Gt,
    Le,
    Ge,
}

impl Display for BinaryOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Add => write!(f, "+"),
            Self::Sub => write!(f, "-"),
            Self::Mul => write!(f, "*"),
            Self::Div => write!(f, "/"),
            Self::Mod => write!(f, "%"),
            Self::Or => write!(f, "||"),
            Self::And => write!(f, "&&"),
            Self::Eq => write!(f, "=="),
            Self::Ne => write!(f, "!="),
            Self::Lt => write!(f, "<"),
            Self::Gt => write!(f, ">"),
            Self::Le => write!(f, "<="),
            Self::Ge => write!(f, ">="),
        }
    }
}

#[non_exhaustive]
#[derive(Debug)]
pub enum BType {
    Int,
}

impl Display for BType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Int => write!(f, "int"),
        }
    }
}

#[derive(Debug)]
pub enum BlockItem {
    Stmt(Stmt),
    Decl(Decl),
}

impl Display for BlockItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Stmt(stmt) => write!(f, "{}", stmt),
            Self::Decl(decl) => write!(f, "{}", decl),
        }
    }
}

#[non_exhaustive]
#[derive(Debug)]
pub enum Decl {
    Const(ConstDecl),
    Var(VarDecl),
}

impl Display for Decl {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Const(const_decl) => write!(f, "{}", const_decl),
            Self::Var(var_decl) => write!(f, "{}", var_decl),
        }
    }
}

#[derive(Debug)]
pub struct ConstDecl {
    pub btype: BType,
    pub defs: Vec<ConstDef>,
}

impl Display for ConstDecl {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "const {} ", self.btype)?;
        for (i, def) in self.defs.iter().enumerate() {
            if i > 0 {
                write!(f, ", ")?;
            }
            write!(f, "{}", def)?;
        }
        write!(f, ";")?;
        Ok(())
    }
}

#[derive(Debug)]
pub struct VarDecl {
    pub btype: BType,
    pub defs: Vec<VarDef>,
}

impl Display for VarDecl {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} ", self.btype)?;
        for (i, def) in self.defs.iter().enumerate() {
            if i > 0 {
                write!(f, ", ")?;
            }
            write!(f, "{}", def)?;
        }
        write!(f, ";")?;
        Ok(())
    }
}

#[derive(Debug)]
pub struct ConstDef {
    pub id: Ident,
    pub init: ConstInit,
}

impl Display for ConstDef {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} = {}", self.id, self.init)
    }
}

#[derive(Debug)]
pub struct VarDef {
    pub id: Ident,
    pub init: Option<VarInit>,
}

impl Display for VarDef {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.id)?;
        if let Some(init) = &self.init {
            write!(f, " = {}", init)?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct ConstInit {
    pub const_expr: ConstExpr,
}

impl Display for ConstInit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.const_expr)
    }
}

#[derive(Debug, Clone)]
pub struct VarInit {
    pub expr: Expr,
}

impl Display for VarInit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.expr)
    }
}

#[derive(Debug, Clone)]
pub struct ConstExpr {
    pub expr: Expr,
}

impl Display for ConstExpr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.expr)
    }
}