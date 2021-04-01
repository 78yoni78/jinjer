use super::*;

#[derive(Debug, Default)]
pub struct VM {
    pub instructions: Vec<Inst>,
    pub lp: usize,
    pub stack: Vec<Value>,
    pub constants: Vec<Value>,
}

macro_rules! check_arguments {
    ($self: ident, $amount: expr) => {
        if $self.stack.len() < $amount {
            return Err("Not enough arguments");
        }
    };
}

macro_rules! get_constant {
    ($self: ident, $index: expr) => {
        $self.constants.get($index).ok_or("Bad constant get")?
    };
}

impl VM {
    pub fn current_inst(&self) -> Inst {
        self.instructions[self.lp]
    }

    pub fn add_constant(&mut self, value: Value) -> usize {
        self.constants.push(value);
        self.constants.len() - 1
    }

    pub fn step(&mut self) -> Result<(), &str> {
        use Inst::*;
        unsafe {
            match self.current_inst() {
                Nop => (),
                Add => {
                    check_arguments!(self, 2);
                    let i2 = self.stack.pop().unwrap().int;
                    let i1 = self.stack.pop().unwrap().int;
                    self.stack.push(Value::int(i1 + i2));
                },
                GetConst(index) => {
                    self.stack.push(*get_constant!(self, index));
                },
            }
            self.lp += 1;
            Ok(())
        }
    }

    pub fn run(mut self) -> Result<Vec<Value>, String> {
        while self.lp < self.instructions.len() {
            self.step()?;
        }
        Ok(self.stack)
    }
}