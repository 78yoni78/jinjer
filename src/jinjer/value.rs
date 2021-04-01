#[derive(Clone, Copy)]
pub union Value {
    pub int: i32,
}

impl Value {
    pub fn int(int: i32) -> Self { Self { int } }
}

impl std::fmt::Debug for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        unsafe {
            let data = self as *const _ as *const u8;
            let bytes: &[u8] = std::slice::from_raw_parts(data, std::mem::size_of::<Self>());
            f.write_fmt(format_args!("{:?}", bytes))
        }
    }
}