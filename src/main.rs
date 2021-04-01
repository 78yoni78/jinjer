#[macro_use]
mod jinjer;

use jinjer::*;
use Inst::*;

fn main() {
    let mut vm = jinjer::VM::default();
    emit!(vm, [
        Nop, Nop,
        get_const Value::int(2),
        Nop, Nop, Nop,
        get_const Value::int(3),
        get_const Value::int(1),
        Add
    ]);
    println!("Hello, world! {:?}", vm.run());
}
