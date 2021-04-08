//  macros come first
#[macro_use]
pub mod bytecode_macros;

mod inst;
mod value;
mod vm;
pub mod tokenizer;
pub mod parser;

pub use inst::Inst;
pub use value::Value;
pub use vm::VM; 

fn main() {
    let s = "x + y";
    let mut tokenizer = tokenizer::Tokenizer::from_reader(s.as_bytes()).unwrap();
    let expr = parser::parse_expr(&mut tokenizer).unwrap();
    println!("Expr: {:?}", expr);
    // let mut vm = jinjer::VM::default();
    // emit!(vm, [
    //     Nop, Nop,
    //     get_const Value::int(2),
    //     Nop, Nop, Nop,
    //     get_const Value::int(3),
    //     get_const Value::int(1),
    //     Add,
    //     Mul,
    // ]);
    // println!("Hello, world! {:?}", vm.run());
}
