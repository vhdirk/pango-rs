// This file was generated by gir (f00d658) from gir-files (0bcaef9)
// DO NOT EDIT

use FontFace;
use ffi;
use glib::object::IsA;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use std::mem;
use std::ptr;

glib_wrapper! {
    pub struct FontFamily(Object<ffi::PangoFontFamily>);

    match fn {
        get_type => || ffi::pango_font_family_get_type(),
    }
}

pub trait FontFamilyExt {
    fn get_name(&self) -> Option<String>;

    fn is_monospace(&self) -> bool;

    fn list_faces(&self) -> Vec<FontFace>;
}

impl<O: IsA<FontFamily>> FontFamilyExt for O {
    fn get_name(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::pango_font_family_get_name(self.to_glib_none().0))
        }
    }

    fn is_monospace(&self) -> bool {
        unsafe {
            from_glib(ffi::pango_font_family_is_monospace(self.to_glib_none().0))
        }
    }

    fn list_faces(&self) -> Vec<FontFace> {
        unsafe {
            let mut faces = ptr::null_mut();
            let mut n_faces = mem::uninitialized();
            ffi::pango_font_family_list_faces(self.to_glib_none().0, &mut faces, &mut n_faces);
            FromGlibContainer::from_glib_container_num(faces, n_faces as usize)
        }
    }
}
