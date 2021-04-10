use crate::{
    parser,
    inst::Inst,
    vm::VM,
    value::Value,
};
use parser::{Expr};

fn bi_oper_inst(oper: parser::BiOper) -> Inst {
    use parser::BiOper::*;
    match oper {
        Add => Inst::Add,
        Sub => Inst::Sub,
        Mul => Inst::Mul,
        Div => Inst::Div,
        Mod => Inst::Mod,
    }
}

pub fn generate(vm: &mut VM, expr: &Expr) {
    use Expr::*;
    match expr {
        &IntLiteral(i) => emit!(vm, [get_const Value::int(i)]),
        &BiOper(oper, ref b) => {
            let (left, right) = &**b;
            let operation_inst = bi_oper_inst(oper);

            generate(vm, left);
            generate(vm, right);
            emit!(vm, [operation_inst]);
        },
        Var(_) | Let(_, _) => todo!("Implement"), 
    }
}

