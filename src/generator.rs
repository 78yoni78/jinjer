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

type Variables = std::collections::HashMap<String, usize>;

fn generate_inner(vm: &mut VM, expr: &Expr, map: &mut Variables) {
    use Expr::*;
    match expr {
        &IntLiteral(i) => emit!(vm, [get_const Value::int(i)]),
        &BiOper(oper, ref b) => {
            let (left, right) = &**b;
            let operation_inst = bi_oper_inst(oper);
            
            generate_inner(vm, left, map);
            generate_inner(vm, right, map);
            emit!(vm, [operation_inst]);
        },
        Let(name, b) => {
            let (body, ret) = &**b;
            
            //  Put the body's value in a variable
            generate_inner(vm, body, map);
            emit!(vm, [ Inst::Var ]);
            
            //  Remember it when running the return
            map.insert(name.clone(), map.len());
            generate_inner(vm, ret, map);
            map.remove(name);

            //  Free the variable
            emit!(vm, [ Inst::EndVar ]);
        },
        Var(name) => {
            let num = map.get(name).unwrap();
            emit!(vm, [Inst::GetVar(map.len() - 1 - num)]);
        },
    }
}


pub fn generate(vm: &mut VM, expr: &Expr) { 
    generate_inner(vm, expr, &mut Variables::default());
}