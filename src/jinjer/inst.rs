#[derive(Debug, Clone, Copy)]
pub enum Inst {
    Nop,
    GetConst(usize),
    Add,
    Sub,
    Mul,
    Mod,
    Div,
}