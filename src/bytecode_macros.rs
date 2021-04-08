#[macro_export]
macro_rules! emit {
    ($self: ident) => {};
    ($self: ident, []) => {};
    ($self: ident, [get_const $value: expr $(, $($rest: tt)*)?]) => {
        let x = $self.add_constant($value);
        emit!($self, [crate::Inst::GetConst(x)]);
        $(emit!($self, [$($rest)*]);)?
    };
    ($self: ident, [$inst: expr $(, $($rest: tt)*)?]) => {
        $self.instructions.push($inst);
        $(emit!($self, [$($rest)*]);)?
    };
}