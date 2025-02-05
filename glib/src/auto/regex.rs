// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::{translate::*, Error, RegexCompileFlags, RegexMatchFlags};

crate::wrapper! {
    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct Regex(Shared<ffi::GRegex>);

    match fn {
        ref => |ptr| ffi::g_regex_ref(ptr),
        unref => |ptr| ffi::g_regex_unref(ptr),
        type_ => || ffi::g_regex_get_type(),
    }
}

impl Regex {
    #[doc(alias = "g_regex_new")]
    pub fn new(
        pattern: &str,
        compile_options: RegexCompileFlags,
        match_options: RegexMatchFlags,
    ) -> Result<Option<Regex>, crate::Error> {
        unsafe {
            let mut error = std::ptr::null_mut();
            let ret = ffi::g_regex_new(
                pattern.to_glib_none().0,
                compile_options.into_glib(),
                match_options.into_glib(),
                &mut error,
            );
            if error.is_null() {
                Ok(from_glib_full(ret))
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    #[doc(alias = "g_regex_get_capture_count")]
    #[doc(alias = "get_capture_count")]
    pub fn capture_count(&self) -> i32 {
        unsafe { ffi::g_regex_get_capture_count(self.to_glib_none().0) }
    }

    #[doc(alias = "g_regex_get_compile_flags")]
    #[doc(alias = "get_compile_flags")]
    pub fn compile_flags(&self) -> RegexCompileFlags {
        unsafe { from_glib(ffi::g_regex_get_compile_flags(self.to_glib_none().0)) }
    }

    #[doc(alias = "g_regex_get_has_cr_or_lf")]
    #[doc(alias = "get_has_cr_or_lf")]
    pub fn has_cr_or_lf(&self) -> bool {
        unsafe { from_glib(ffi::g_regex_get_has_cr_or_lf(self.to_glib_none().0)) }
    }

    #[doc(alias = "g_regex_get_match_flags")]
    #[doc(alias = "get_match_flags")]
    pub fn match_flags(&self) -> RegexMatchFlags {
        unsafe { from_glib(ffi::g_regex_get_match_flags(self.to_glib_none().0)) }
    }

    #[doc(alias = "g_regex_get_max_backref")]
    #[doc(alias = "get_max_backref")]
    pub fn max_backref(&self) -> i32 {
        unsafe { ffi::g_regex_get_max_backref(self.to_glib_none().0) }
    }

    #[doc(alias = "g_regex_get_max_lookbehind")]
    #[doc(alias = "get_max_lookbehind")]
    pub fn max_lookbehind(&self) -> i32 {
        unsafe { ffi::g_regex_get_max_lookbehind(self.to_glib_none().0) }
    }

    #[doc(alias = "g_regex_get_pattern")]
    #[doc(alias = "get_pattern")]
    pub fn pattern(&self) -> crate::GString {
        unsafe { from_glib_none(ffi::g_regex_get_pattern(self.to_glib_none().0)) }
    }

    //#[doc(alias = "g_regex_replace_eval")]
    //pub fn replace_eval(&self, string: &[&str], start_position: i32, match_options: RegexMatchFlags, eval: /*Unimplemented*/Fn(&MatchInfo, /*Ignored*/String) -> bool, user_data: /*Unimplemented*/Option<Basic: Pointer>) -> Result<crate::GString, crate::Error> {
    //    unsafe { TODO: call ffi:g_regex_replace_eval() }
    //}
}
