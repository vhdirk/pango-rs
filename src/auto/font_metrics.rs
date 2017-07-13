// This file was generated by gir (4b09025) from gir-files (71d73f0)
// DO NOT EDIT

use ffi;
use glib::translate::*;

glib_wrapper! {
    pub struct FontMetrics(Shared<ffi::PangoFontMetrics>);

    match fn {
        ref => |ptr| ffi::pango_font_metrics_ref(ptr),
        unref => |ptr| ffi::pango_font_metrics_unref(ptr),
    }
}

impl FontMetrics {
    pub fn new() -> FontMetrics {
        unsafe {
            from_glib_full(ffi::pango_font_metrics_new())
        }
    }

    pub fn get_approximate_char_width(&self) -> i32 {
        unsafe {
            ffi::pango_font_metrics_get_approximate_char_width(self.to_glib_none().0)
        }
    }

    pub fn get_approximate_digit_width(&self) -> i32 {
        unsafe {
            ffi::pango_font_metrics_get_approximate_digit_width(self.to_glib_none().0)
        }
    }

    pub fn get_ascent(&self) -> i32 {
        unsafe {
            ffi::pango_font_metrics_get_ascent(self.to_glib_none().0)
        }
    }

    pub fn get_descent(&self) -> i32 {
        unsafe {
            ffi::pango_font_metrics_get_descent(self.to_glib_none().0)
        }
    }

    pub fn get_strikethrough_position(&self) -> i32 {
        unsafe {
            ffi::pango_font_metrics_get_strikethrough_position(self.to_glib_none().0)
        }
    }

    pub fn get_strikethrough_thickness(&self) -> i32 {
        unsafe {
            ffi::pango_font_metrics_get_strikethrough_thickness(self.to_glib_none().0)
        }
    }

    pub fn get_underline_position(&self) -> i32 {
        unsafe {
            ffi::pango_font_metrics_get_underline_position(self.to_glib_none().0)
        }
    }

    pub fn get_underline_thickness(&self) -> i32 {
        unsafe {
            ffi::pango_font_metrics_get_underline_thickness(self.to_glib_none().0)
        }
    }
}
