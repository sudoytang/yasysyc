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
    pub stmt: Stmt,
}

impl Display for Block {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{ {} }}", self.stmt)
    }
}



#[non_exhaustive]
#[derive(Debug)]
pub enum Stmt {
    Return(ReturnStmt),
}

impl Display for Stmt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Return(return_stmt) => write!(f, "{}", return_stmt),
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

#[non_exhaustive]
#[derive(Debug)]
pub enum Expr {
    Number(i32),
    Unary(UnaryOp, Box<Expr>),
}

impl Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Number(number) => write!(f, "{}", number),
            Self::Unary(unary_op, expr) => write!(f, "{}{}", unary_op, expr),
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