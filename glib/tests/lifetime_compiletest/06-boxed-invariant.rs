#![allow(unused)]

use std::sync::Once;

#[derive(Clone, Copy)]
#[repr(C)]
pub struct InnerRaw {
    data: *const [u8],
}

glib::wrapper! {
    pub struct TestBoxedRecord<'data>(Boxed<InnerRaw>);

    match fn {
        copy => |ptr| Box::into_raw(Box::new(*ptr)),
        free => |ptr| drop(Box::from_raw(ptr)),
    }
}

impl<'data> TestBoxedRecord<'data> {
    fn new(data: &'data [u8]) -> Self {
        unsafe { glib::translate::from_glib_none::<*const InnerRaw, Self>(&InnerRaw { data }) }
    }

    fn data(&self) -> &'data [u8] {
        unsafe { &*self.inner.data }
    }
}

fn not_covariant<'a>(record: TestBoxedRecord<'static>) -> TestBoxedRecord<'a> {
    record
}

fn main() {}
