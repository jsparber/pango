// This file was generated by gir (https://github.com/gtk-rs/gir @ 7f5a2b5)
// from gir-files (https://github.com/gtk-rs/gir-files @ 4740f5e)
// DO NOT EDIT

use AttrList;
use Direction;
use Error;
use Rectangle;
use Stretch;
use Style;
use Variant;
use Weight;
use ffi;
use glib;
use glib::translate::*;
use std::mem;
use std::ptr;


//pub fn break_(text: &str, analysis: /*Ignored*/&mut Analysis, attrs: /*Ignored*/&[&LogAttr]) {
//    unsafe { TODO: call ffi::pango_break() }
//}

#[cfg_attr(feature = "v1_38", deprecated)]
pub fn config_key_get(key: &str) -> Option<String> {
    unsafe {
        from_glib_full(ffi::pango_config_key_get(key.to_glib_none().0))
    }
}

#[cfg_attr(feature = "v1_38", deprecated)]
pub fn config_key_get_system(key: &str) -> Option<String> {
    unsafe {
        from_glib_full(ffi::pango_config_key_get_system(key.to_glib_none().0))
    }
}

//pub fn default_break<'a, P: Into<Option<&'a /*Ignored*/Analysis>>>(text: &str, analysis: P, attrs: /*Ignored*/&mut LogAttr, attrs_len: i32) {
//    unsafe { TODO: call ffi::pango_default_break() }
//}

pub fn extents_to_pixels<'a, 'b, P: Into<Option<&'a Rectangle>>, Q: Into<Option<&'b Rectangle>>>(inclusive: P, nearest: Q) {
    let inclusive = inclusive.into();
    let nearest = nearest.into();
    unsafe {
        ffi::pango_extents_to_pixels(mut_override(inclusive.to_glib_none().0), mut_override(nearest.to_glib_none().0));
    }
}

pub fn find_base_dir(text: &str) -> Direction {
    let length = text.len() as i32;
    unsafe {
        from_glib(ffi::pango_find_base_dir(text.to_glib_none().0, length))
    }
}

//#[cfg_attr(feature = "v1_38", deprecated)]
//pub fn find_map(language: &mut Language, engine_type_id: u32, render_type_id: u32) -> /*Ignored*/Option<Map> {
//    unsafe { TODO: call ffi::pango_find_map() }
//}

pub fn find_paragraph_boundary(text: &str) -> (i32, i32) {
    let length = text.len() as i32;
    unsafe {
        let mut paragraph_delimiter_index = mem::uninitialized();
        let mut next_paragraph_start = mem::uninitialized();
        ffi::pango_find_paragraph_boundary(text.to_glib_none().0, length, &mut paragraph_delimiter_index, &mut next_paragraph_start);
        (paragraph_delimiter_index, next_paragraph_start)
    }
}

#[cfg_attr(feature = "v1_38", deprecated)]
pub fn get_lib_subdirectory() -> Option<String> {
    unsafe {
        from_glib_none(ffi::pango_get_lib_subdirectory())
    }
}

//pub fn get_log_attrs(text: &str, level: i32, language: &mut Language, log_attrs: /*Ignored*/&[&LogAttr]) {
//    unsafe { TODO: call ffi::pango_get_log_attrs() }
//}

#[cfg_attr(feature = "v1_38", deprecated)]
pub fn get_sysconf_subdirectory() -> Option<String> {
    unsafe {
        from_glib_none(ffi::pango_get_sysconf_subdirectory())
    }
}

pub fn is_zero_width(ch: char) -> bool {
    unsafe {
        from_glib(ffi::pango_is_zero_width(ch.to_glib()))
    }
}

//pub fn itemize<'a, P: Into<Option<&'a /*Ignored*/AttrIterator>>>(context: &Context, text: &str, start_index: i32, length: i32, attrs: &AttrList, cached_iter: P) -> /*Ignored*/Vec<Item> {
//    unsafe { TODO: call ffi::pango_itemize() }
//}

//pub fn itemize_with_base_dir<'a, P: Into<Option<&'a /*Ignored*/AttrIterator>>>(context: &Context, base_dir: Direction, text: &str, start_index: i32, length: i32, attrs: &AttrList, cached_iter: P) -> /*Ignored*/Vec<Item> {
//    unsafe { TODO: call ffi::pango_itemize_with_base_dir() }
//}

#[cfg_attr(feature = "v1_32", deprecated)]
pub fn lookup_aliases(fontname: &str) -> Vec<String> {
    unsafe {
        let mut families = ptr::null_mut();
        let mut n_families = mem::uninitialized();
        ffi::pango_lookup_aliases(fontname.to_glib_none().0, &mut families, &mut n_families);
        FromGlibContainer::from_glib_full_num(families, n_families as usize)
    }
}

//#[cfg(any(feature = "v1_31", feature = "dox"))]
//pub fn markup_parser_finish(context: /*Ignored*/&glib::MarkupParseContext) -> Result<(AttrList, String, char), Error> {
//    unsafe { TODO: call ffi::pango_markup_parser_finish() }
//}

//#[cfg(any(feature = "v1_31", feature = "dox"))]
//pub fn markup_parser_new(accel_marker: char) -> /*Ignored*/Option<glib::MarkupParseContext> {
//    unsafe { TODO: call ffi::pango_markup_parser_new() }
//}

//#[cfg_attr(feature = "v1_38", deprecated)]
//pub fn module_register(module: /*Ignored*/&mut IncludedModule) {
//    unsafe { TODO: call ffi::pango_module_register() }
//}

#[cfg_attr(feature = "v1_38", deprecated)]
pub fn parse_enum<'a, P: Into<Option<&'a str>>>(type_: glib::types::Type, str: P, warn: bool) -> Option<(i32, String)> {
    let str = str.into();
    let str = str.to_glib_none();
    unsafe {
        let mut value = mem::uninitialized();
        let mut possible_values = ptr::null_mut();
        let ret = from_glib(ffi::pango_parse_enum(type_.to_glib(), str.0, &mut value, warn.to_glib(), &mut possible_values));
        if ret { Some((value, from_glib_full(possible_values))) } else { None }
    }
}

pub fn parse_markup(markup_text: &str, accel_marker: char) -> Result<(AttrList, String, char), Error> {
    let length = markup_text.len() as i32;
    unsafe {
        let mut attr_list = ptr::null_mut();
        let mut text = ptr::null_mut();
        let mut accel_char = mem::uninitialized();
        let mut error = ptr::null_mut();
        let _ = ffi::pango_parse_markup(markup_text.to_glib_none().0, length, accel_marker.to_glib(), &mut attr_list, &mut text, &mut accel_char, &mut error);
        if error.is_null() { Ok((from_glib_full(attr_list), from_glib_full(text), from_glib(accel_char))) } else { Err(from_glib_full(error)) }
    }
}

pub fn parse_stretch(str: &str, warn: bool) -> Option<Stretch> {
    unsafe {
        let mut stretch = mem::uninitialized();
        let ret = from_glib(ffi::pango_parse_stretch(str.to_glib_none().0, &mut stretch, warn.to_glib()));
        if ret { Some(from_glib(stretch)) } else { None }
    }
}

pub fn parse_style(str: &str, warn: bool) -> Option<Style> {
    unsafe {
        let mut style = mem::uninitialized();
        let ret = from_glib(ffi::pango_parse_style(str.to_glib_none().0, &mut style, warn.to_glib()));
        if ret { Some(from_glib(style)) } else { None }
    }
}

pub fn parse_variant(str: &str, warn: bool) -> Option<Variant> {
    unsafe {
        let mut variant = mem::uninitialized();
        let ret = from_glib(ffi::pango_parse_variant(str.to_glib_none().0, &mut variant, warn.to_glib()));
        if ret { Some(from_glib(variant)) } else { None }
    }
}

pub fn parse_weight(str: &str, warn: bool) -> Option<Weight> {
    unsafe {
        let mut weight = mem::uninitialized();
        let ret = from_glib(ffi::pango_parse_weight(str.to_glib_none().0, &mut weight, warn.to_glib()));
        if ret { Some(from_glib(weight)) } else { None }
    }
}

pub fn quantize_line_geometry(thickness: &mut i32, position: &mut i32) {
    unsafe {
        ffi::pango_quantize_line_geometry(thickness, position);
    }
}

//#[cfg_attr(feature = "v1_38", deprecated)]
//pub fn read_line<P: Into<Option</*Unimplemented*/Fundamental: Pointer>>>(stream: P, str: /*Ignored*/glib::String) -> i32 {
//    unsafe { TODO: call ffi::pango_read_line() }
//}

//pub fn reorder_items(logical_items: /*Ignored*/&[&Item]) -> /*Ignored*/Vec<Item> {
//    unsafe { TODO: call ffi::pango_reorder_items() }
//}

//#[cfg_attr(feature = "v1_38", deprecated)]
//pub fn scan_int(pos: /*Unimplemented*/String) -> Option<i32> {
//    unsafe { TODO: call ffi::pango_scan_int() }
//}

//#[cfg_attr(feature = "v1_38", deprecated)]
//pub fn scan_string(pos: /*Unimplemented*/String, out: /*Ignored*/glib::String) -> bool {
//    unsafe { TODO: call ffi::pango_scan_string() }
//}

//#[cfg_attr(feature = "v1_38", deprecated)]
//pub fn scan_word(pos: /*Unimplemented*/String, out: /*Ignored*/glib::String) -> bool {
//    unsafe { TODO: call ffi::pango_scan_word() }
//}

//pub fn shape(text: &str, analysis: /*Ignored*/&Analysis, glyphs: &mut GlyphString) {
//    unsafe { TODO: call ffi::pango_shape() }
//}

//#[cfg(any(feature = "v1_32", feature = "dox"))]
//pub fn shape_full<'a, P: Into<Option<&'a str>>>(item_text: &str, paragraph_text: P, analysis: /*Ignored*/&Analysis, glyphs: &mut GlyphString) {
//    unsafe { TODO: call ffi::pango_shape_full() }
//}

//#[cfg_attr(feature = "v1_38", deprecated)]
//pub fn skip_space(pos: /*Unimplemented*/String) -> bool {
//    unsafe { TODO: call ffi::pango_skip_space() }
//}

#[cfg_attr(feature = "v1_38", deprecated)]
pub fn split_file_list(str: &str) -> Vec<String> {
    unsafe {
        FromGlibPtrContainer::from_glib_full(ffi::pango_split_file_list(str.to_glib_none().0))
    }
}

#[cfg_attr(feature = "v1_38", deprecated)]
pub fn trim_string(str: &str) -> Option<String> {
    unsafe {
        from_glib_full(ffi::pango_trim_string(str.to_glib_none().0))
    }
}

pub fn unichar_direction(ch: char) -> Direction {
    unsafe {
        from_glib(ffi::pango_unichar_direction(ch.to_glib()))
    }
}

pub fn units_from_double(d: f64) -> i32 {
    unsafe {
        ffi::pango_units_from_double(d)
    }
}

pub fn units_to_double(i: i32) -> f64 {
    unsafe {
        ffi::pango_units_to_double(i)
    }
}

pub fn version() -> i32 {
    unsafe {
        ffi::pango_version()
    }
}

pub fn version_check(required_major: i32, required_minor: i32, required_micro: i32) -> Option<String> {
    unsafe {
        from_glib_none(ffi::pango_version_check(required_major, required_minor, required_micro))
    }
}

pub fn version_string() -> Option<String> {
    unsafe {
        from_glib_none(ffi::pango_version_string())
    }
}
