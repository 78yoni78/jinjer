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
    ($self: expr, [$inst: expr $(, $($rest: tt)*)?]) => {
        $self.instructions.push($inst);
        $(emit!($self, [$($rest)*]);)?
    };
}