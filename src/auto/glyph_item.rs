// This file was generated by gir (f00d658) from gir-files (0bcaef9)
// DO NOT EDIT

use AttrList;
use ffi;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use std::mem;
use std::ptr;

glib_wrapper! {
    pub struct GlyphItem(Boxed<ffi::PangoGlyphItem>);

    match fn {
        copy => |ptr| ffi::pango_glyph_item_copy(mut_override(ptr)),
        free => |ptr| ffi::pango_glyph_item_free(ptr),
        get_type => || ffi::pango_glyph_item_get_type(),
    }
}

impl GlyphItem {
    pub fn apply_attrs(&mut self, text: &str, list: &AttrList) -> Vec<GlyphItem> {
        unsafe {
            FromGlibPtrContainer::from_glib_full(ffi::pango_glyph_item_apply_attrs(self.to_glib_none_mut().0, text.to_glib_none().0, list.to_glib_none().0))
        }
    }

    //pub fn get_logical_widths(&mut self, text: &str, logical_widths: &[i32]) {
    //    unsafe { TODO: call ffi::pango_glyph_item_get_logical_widths() }
    //}

    //pub fn letter_space(&mut self, text: &str, log_attrs: /*Ignored*/&[&LogAttr], letter_spacing: i32) {
    //    unsafe { TODO: call ffi::pango_glyph_item_letter_space() }
    //}

    pub fn split(&mut self, text: &str, split_index: i32) -> Option<GlyphItem> {
        unsafe {
            from_glib_full(ffi::pango_glyph_item_split(self.to_glib_none_mut().0, text.to_glib_none().0, split_index))
        }
    }
}
