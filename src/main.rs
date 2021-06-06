use jinjer::*;

fn main() {
    let code = "\"Hello wordl!\"";
    println!("Code: {}", code);
    println!();
    let mut vm = VM::default();
    let mut tokenizer = tokenizer::Tokenizer::from_reader(code.as_bytes()).unwrap();
    let expr = parser::parse_expr(&mut tokenizer).unwrap();
    println!("Expr: {:?}", expr);
    println!();
    generator::generate(&mut vm, &expr);
    println!("Running: {:?}", vm);
    println!();
    println!("Result: {:?}", vm.run());
}
