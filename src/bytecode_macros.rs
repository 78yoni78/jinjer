#[macro_export]
macro_rules! emit {
    ($self: expr) => {};
    ($self: expr, []) => {};
    ($self: expr, [get_const $value: expr $(, $($rest: tt)*)?]) => {
        {
            let x = $self.add_constant($value);
            emit!($self, [crate::Inst::GetConst(x)]);
            $(emit!($self, [$($rest)*]);)?
        }
    };
    ($self: expr, [get_str $string: expr $(, $($rest: tt)*)?]) => {
        {
            let char_values = $string.as_bytes().chunks(4).map(|c| {
                let mut v = crate::Value::int(0);
                if 0 < c.len() { v._4bytes.0 = c[0]; }
                if 1 < c.len() { v._4bytes.1 = c[1]; }
                if 2 < c.len() { v._4bytes.2 = c[2]; }
                if 3 < c.len() { v._4bytes.3 = c[3]; }
                v
            });
            let index = $self.add_constants(
                std::iter::once(Value { usize: $string.len() }).chain(char_values)
            );
            emit!($self, [crate::Inst::GetStr(index)])
            $(emit!($self, [$($rest)*]);)?
        }
    };
    ($self: expr, [$inst: expr $(, $($rest: tt)*)?]) => {
        $self.instructions.push($inst);
        $(emit!($self, [$($rest)*]);)?
    };
}