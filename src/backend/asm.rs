use std::fmt::Display;

pub enum AsmLine {
    Directive(Directive),
    Instruction(Instruction),
    Label(String),
    Comment(String),
}

impl Display for AsmLine {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Directive(directive) => write!(f, "{}", directive),
            Self::Instruction(instruction) => write!(f, "{}", instruction),
            Self::Label(label) => write!(f, "{}:", label),
            Self::Comment(comment) => write!(f, "# {}", comment),
        }
    }
}

#[non_exhaustive]
pub enum Instruction {
    Li { reg: Reg, imm: i32 },
    // Arithmetic
    Sub { rd: Reg, rs1: Reg, rs2: Reg },
    // Logical
    Seqz { rd: Reg, rs: Reg },  // set if equal to zero
    Snez { rd: Reg, rs: Reg },  // set if not equal to zero
    // Move
    Mv { rd: Reg, rs: Reg },

    Ret,
}

impl Display for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Li { reg, imm } => write!(f, "  li {}, {}", reg, imm),
            Self::Sub { rd, rs1, rs2 } => write!(f, "  sub {}, {}, {}", rd, rs1, rs2),
            Self::Seqz { rd, rs } => write!(f, "  seqz {}, {}", rd, rs),
            Self::Snez { rd, rs } => write!(f, "  snez {}, {}", rd, rs),
            Self::Mv { rd, rs } => write!(f, "  mv {}, {}", rd, rs),
            Self::Ret => write!(f, "  ret"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Reg {
    // Zero register
    Zero,
    // Return address
    Ra,
    // Stack pointer
    Sp,
    // Global pointer
    Gp,
    // Thread pointer
    Tp,
    // Temporaries
    T0, T1, T2, T3, T4, T5, T6,
    // Saved registers
    S0, S1, S2, S3, S4, S5, S6, S7, S8, S9, S10, S11,
    // Function arguments / return values
    A0, A1, A2, A3, A4, A5, A6, A7,
}

impl Display for Reg {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Zero => write!(f, "zero"),
            Self::Ra => write!(f, "ra"),
            Self::Sp => write!(f, "sp"),
            Self::Gp => write!(f, "gp"),
            Self::Tp => write!(f, "tp"),
            Self::T0 => write!(f, "t0"),
            Self::T1 => write!(f, "t1"),
            Self::T2 => write!(f, "t2"),
            Self::T3 => write!(f, "t3"),
            Self::T4 => write!(f, "t4"),
            Self::T5 => write!(f, "t5"),
            Self::T6 => write!(f, "t6"),
            Self::S0 => write!(f, "s0"),
            Self::S1 => write!(f, "s1"),
            Self::S2 => write!(f, "s2"),
            Self::S3 => write!(f, "s3"),
            Self::S4 => write!(f, "s4"),
            Self::S5 => write!(f, "s5"),
            Self::S6 => write!(f, "s6"),
            Self::S7 => write!(f, "s7"),
            Self::S8 => write!(f, "s8"),
            Self::S9 => write!(f, "s9"),
            Self::S10 => write!(f, "s10"),
            Self::S11 => write!(f, "s11"),
            Self::A0 => write!(f, "a0"),
            Self::A1 => write!(f, "a1"),
            Self::A2 => write!(f, "a2"),
            Self::A3 => write!(f, "a3"),
            Self::A4 => write!(f, "a4"),
            Self::A5 => write!(f, "a5"),
            Self::A6 => write!(f, "a6"),
            Self::A7 => write!(f, "a7"),
        }
    }
}


pub enum Directive {
    Section(Section),
    Global(String),
}

impl Display for Directive {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Section(section) => write!(f, "{}", section),
            Self::Global(symbol) => write!(f, ".globl {}", symbol),
        }
    }
}

pub enum Section {
    Text,
    Data,
}

impl Display for Section {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Text => write!(f, ".text"),
            Self::Data => write!(f, ".data"),
        }
    }
}