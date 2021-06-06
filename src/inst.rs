#[derive(Debug, Clone, Copy)]
pub enum Inst {
    Nop,
    GetConst(usize),
    GetStr(usize),
    Add,
    Sub,
    Mul,
    Mod,
    Div,
    //  Variable instructions
    Var,
    GetVar(usize),
    EndVar,
}