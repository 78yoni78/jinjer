use std::{
    alloc::{alloc, dealloc, Layout},
    ptr::NonNull,
    cell::Cell,
};

struct Header {
    rc: Cell<u32>,
    layout: Layout,
}

#[derive(Copy)]
pub struct Object(NonNull<Header>);


impl Object {
    unsafe fn alloc(layout: Layout) -> Option<NonNull<Header>> {
        let payload_layout = 
            Layout::new::<Header>()
            .extend(layout).ok()?.0
            .pad_to_align();

        let mut s = NonNull::new(alloc(payload_layout) as *mut Header)?;
        s.as_mut().rc = Cell::new(1);
        s.as_mut().layout = layout;
        Some(s)
    }

    unsafe fn dealloc(payload: NonNull<Header>) {
        let payload_layout = 
            Layout::new::<Header>()
            .extend(payload.as_ref().layout).unwrap().0
            .pad_to_align();

        dealloc(payload.as_ptr() as *mut u8, payload_layout);
    }

    pub unsafe fn new(layout: Layout) -> Option<Self> {
        let payload = Self::alloc(layout)?;
        Some(Self(payload))
    }

    pub fn drop(self) {
        unsafe {
            let rc = &self.0.as_ref().rc;
            rc.set(rc.get() - 1);

            if rc.get() == 0 {
                Self::dealloc(self.0);
            }
        }
    }
}

impl Clone for Object {
    fn clone(&self) -> Self {
        unsafe {
            let rc = &self.0.as_ref().rc;
            rc.set(rc.get() + 1);
            Self(self.0)
        }
    }
}
