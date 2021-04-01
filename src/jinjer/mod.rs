//  macros come first
#[macro_use]
pub mod bytecode_macros;

mod inst;
mod value;
mod vm;

pub use inst::Inst;
pub use value::Value;
pub use vm::VM; 