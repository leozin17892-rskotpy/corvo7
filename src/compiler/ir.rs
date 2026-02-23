use crate::compiler::parser::*;
use crate::compiler::parser::Type;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Temp(pub usize);

#[derive(Debug, Clone)]
pub struct Label(pub String);

#[derive(Debug, Clone)]
pub enum Instr {
    ConstInt { dst: Temp, value: i128 },
    ConstBool { dst: Temp, value: bool },

    Load { dst: Temp, name: String },
    Store { name: String, src: Temp },

    Add { dst: Temp, lhs: Temp, rhs: Temp },
    Sub { dst: Temp, lhs: Temp, rhs: Temp },
    Mul { dst: Temp, lhs: Temp, rhs: Temp },
    Div { dst: Temp, lhs: Temp, rhs: Temp },

    Eq { dst: Temp, lhs: Temp, rhs: Temp },
    Greater { dst: Temp, lhs: Temp, rhs: Temp },
    Less { dst: Temp, lhs: Temp, rhs: Temp },
    GreaterEq { dst: Temp, lhs: Temp, rhs: Temp },
    LessEq { dst: Temp, lhs: Temp, rhs: Temp },

    Label(Label),
    Print{temp: Temp, ty: Type},
    Jump(Label),
    JumpIfFalse { cond: Temp, label: Label },

    Call { dst: Option<Temp>, name: String, args: Vec<Temp> },

    Return(Option<Temp>),
}

#[derive(Debug)]
pub struct IRProgram {
    pub instructions: Vec<Instr>,
}