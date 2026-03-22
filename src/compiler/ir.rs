use crate::compiler::parser::Type;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Temp{
    pub id: usize,
    pub ty: Type,
}
impl Temp {
    pub fn new(id: usize, ty: Type) -> Self{
        Self{ id, ty }
    }
    
    pub fn ty(&self) -> &Type{
        &self.ty
    }
    
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct FuncId{
    pub id: usize
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Label(pub usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct VarId(pub usize);

#[derive(Debug, Clone)]
pub enum Instr {
    ConstInt { dst: Temp, value: i128 },
    ConstBool { dst: Temp, value: bool },

    Load { dst: Temp, id: VarId },
    Store { var: VarId, src: Temp },

    Add { dst: Temp, lhs: Temp, rhs: Temp },
    Sub { dst: Temp, lhs: Temp, rhs: Temp },
    Mul { dst: Temp, lhs: Temp, rhs: Temp },
    Div { dst: Temp, lhs: Temp, rhs: Temp },

    Eq { dst: Temp, lhs: Temp, rhs: Temp },
    Greater { dst: Temp, lhs: Temp, rhs: Temp },
    Less { dst: Temp, lhs: Temp, rhs: Temp },
    GreaterEq { dst: Temp, lhs: Temp, rhs: Temp },
    LessEq { dst: Temp, lhs: Temp, rhs: Temp },

    Print{temp: Temp},
    Jump(Label),
    JumpIfFalse { cond: Temp, label: Label },

    Call { dst: Option<Temp>, name: FuncId, args: Vec<Temp> },

    Return(Option<Temp>),
}

#[derive(Debug)]
pub struct BasicBlock{
    pub label: Label,
    pub instructions: Vec<Instr>,
}

#[derive(Debug)]
pub struct FunctionIR{
    pub name: String,
    pub blocks: Vec<BasicBlock>
}

#[derive(Debug)]
pub struct IRProgram {
    pub functions: Vec<FunctionIR>,
}