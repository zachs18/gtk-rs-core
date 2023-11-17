#![allow(unused)]

use std::sync::Once;

#[derive(Clone, Copy)]
#[repr(C)]
pub struct InnerRaw {
    data: *const [u8],
}

fn get_test_boxed_record_type() -> glib::ffi::GType {
    static ONCE: Once = Once::new();
    static mut TYPE: glib::ffi::GType = 0;
    unsafe extern "C" fn boxed_copy(v: glib::ffi::gpointer) -> glib::ffi::gpointer {
        let v = &*(v as *mut InnerRaw);
        let copy = Box::new(v.clone());

        Box::into_raw(copy) as glib::ffi::gpointer
    }
    unsafe extern "C" fn boxed_free(v: glib::ffi::gpointer) {
        let v = v as *mut InnerRaw;
        let _ = Box::from_raw(v);
    }
    ONCE.call_once(|| unsafe {
        const NAME: &str = "TestBoxedRecord04\0";
        TYPE = glib::gobject_ffi::g_boxed_type_register_static(
            NAME.as_ptr().cast(),
            Some(boxed_copy),
            Some(boxed_free),
        );
    });
    unsafe { TYPE }
}

glib::wrapper! {
    pub struct TestBoxedRecord<'data>(Boxed<InnerRaw>);

    match fn {
        copy => |ptr| Box::into_raw(Box::new(*ptr)),
        free => |ptr| drop(Box::from_raw(ptr)),
        type_ => || get_test_boxed_record_type(),
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

fn main() {
    use glib::ToValue;

    let data: &'static [u8] = &[1, 2, 3];
    let record = TestBoxedRecord::new(&data);
    let value = record.to_value();
    let record = value
        .get::<TestBoxedRecord<'_>>()
        .expect("contains a TestBoxedRecord");
}
