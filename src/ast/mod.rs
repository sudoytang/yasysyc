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
    pub num: Number,
}

impl Display for ReturnStmt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "return {};", self.num)
    }
}

#[derive(Debug)]
pub struct Number {
    pub value: i32,
}

impl From<i32> for Number {
    fn from(value: i32) -> Self {
        Number { value }
    }
}

impl Display for Number {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value)
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