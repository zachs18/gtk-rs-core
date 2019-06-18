// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use glib::object::Cast;
use glib::translate::*;
use glib::StaticType;
use glib::ToValue;
use gtk_sys;
use std::fmt;
use IMContext;
use InputHints;
use InputPurpose;

glib_wrapper! {
    pub struct IMContextSimple(Object<gtk_sys::GtkIMContextSimple, gtk_sys::GtkIMContextSimpleClass, IMContextSimpleClass>) @extends IMContext;

    match fn {
        get_type => || gtk_sys::gtk_im_context_simple_get_type(),
    }
}

impl IMContextSimple {
    pub fn new() -> IMContextSimple {
        assert_initialized_main_thread!();
        unsafe { IMContext::from_glib_full(gtk_sys::gtk_im_context_simple_new()).unsafe_cast() }
    }
}

impl Default for IMContextSimple {
    fn default() -> Self {
        Self::new()
    }
}

pub struct IMContextSimpleBuilder {
    input_hints: Option<InputHints>,
    input_purpose: Option<InputPurpose>,
}

impl IMContextSimpleBuilder {
    pub fn new() -> Self {
        Self {
            input_hints: None,
            input_purpose: None,
        }
    }

    pub fn build(self) -> IMContextSimple {
        let mut properties: Vec<(&str, &dyn ToValue)> = vec![];
        if let Some(ref input_hints) = self.input_hints {
            properties.push(("input-hints", input_hints));
        }
        if let Some(ref input_purpose) = self.input_purpose {
            properties.push(("input-purpose", input_purpose));
        }
        glib::Object::new(IMContextSimple::static_type(), &properties)
            .expect("object new")
            .downcast()
            .expect("downcast")
    }

    pub fn input_hints(mut self, input_hints: InputHints) -> Self {
        self.input_hints = Some(input_hints);
        self
    }

    pub fn input_purpose(mut self, input_purpose: InputPurpose) -> Self {
        self.input_purpose = Some(input_purpose);
        self
    }
}

pub const NONE_IM_CONTEXT_SIMPLE: Option<&IMContextSimple> = None;

impl fmt::Display for IMContextSimple {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "IMContextSimple")
    }
}
