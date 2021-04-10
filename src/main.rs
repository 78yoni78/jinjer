use jinjer::*;

fn create_vm(code: &str) -> VM {
    let mut vm = VM::default();
    let mut tokenizer = tokenizer::Tokenizer::from_reader(code.as_bytes()).unwrap();
    let expr = parser::parse_expr(&mut tokenizer).unwrap();
    generator::generate(&mut vm, &expr);
    vm
}

fn main() {
    let vm = create_vm("1 + 2 + 4");
    println!("Running: {:?}", vm);
    println!();
    println!("Result: {:?}", vm.run());
}
