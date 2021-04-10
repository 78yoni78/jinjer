use super::*;

#[derive(Debug, Default)]
pub struct VM {
    pub instructions: Vec<Inst>,
    pub lp: usize,
    pub stack: Vec<Value>,
    pub constants: Vec<Value>,
    pub variables: Vec<Value>
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

macro_rules! step_binary_inst {
    ($self: ident, $arg_type: ident, $op: tt) => {
        {
            check_arguments!($self, 2);
            let a2 = $self.stack.pop().unwrap().$arg_type;
            let a1 = $self.stack.pop().unwrap().$arg_type;
            $self.stack.push(Value::$arg_type(a1 $op a2));
        }
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
                Add => step_binary_inst!(self, int, +),
                Sub => step_binary_inst!(self, int, -),
                Mul => step_binary_inst!(self, int, *),
                Mod => step_binary_inst!(self, int, %),
                Div => step_binary_inst!(self, int, /),
                GetConst(index) => {
                    self.stack.push(*get_constant!(self, index));
                },
                Var => {
                    let value = self.stack.pop().ok_or("Stack exhausted")?;
                    self.variables.push(value);
                },
                GetVar(diff) => {
                    let index = self.variables.len() - 1 - diff;
                    self.stack.push(self.variables[index]);
                },
                EndVar => {
                    self.variables.pop().ok_or("Variable stack exhausted")?;
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

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn getting_constants_nop_and_add() {
        use Inst::*;
        
        let mut vm = VM::default();
        emit!(vm, [
            Nop, Nop,
            get_const Value::int(2),
            Nop, Nop, Nop,
            get_const Value::int(3),
            get_const Value::int(1),
            Add
        ]);
        let result = vm.run().unwrap();
        assert_eq!(result.len(), 2);
        unsafe {
            assert_eq!(result[0].int, 2);
            assert_eq!(result[1].int, 4);
        };
    }
}