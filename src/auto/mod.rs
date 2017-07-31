// This file was generated by gir (f00d658) from gir-files (0bcaef9)
// DO NOT EDIT

mod context;
pub use self::context::Context;
pub use self::context::ContextExt;

mod engine_shape;
pub use self::engine_shape::EngineShape;

mod font;
pub use self::font::Font;
pub use self::font::FontExt;

mod font_face;
pub use self::font_face::FontFace;
pub use self::font_face::FontFaceExt;

mod font_family;
pub use self::font_family::FontFamily;
pub use self::font_family::FontFamilyExt;

mod font_map;
pub use self::font_map::FontMap;
pub use self::font_map::FontMapExt;

mod fontset;
pub use self::fontset::Fontset;
pub use self::fontset::FontsetExt;

mod layout;
pub use self::layout::Layout;
pub use self::layout::LayoutExt;

mod renderer;
pub use self::renderer::Renderer;
pub use self::renderer::RendererExt;

mod attr_list;
pub use self::attr_list::AttrList;

mod attribute;
pub use self::attribute::Attribute;

mod color;
pub use self::color::Color;

mod coverage;
pub use self::coverage::Coverage;

mod font_description;
pub use self::font_description::FontDescription;

mod font_metrics;
pub use self::font_metrics::FontMetrics;

mod glyph_item;
pub use self::glyph_item::GlyphItem;

mod glyph_item_iter;
pub use self::glyph_item_iter::GlyphItemIter;

mod glyph_string;
pub use self::glyph_string::GlyphString;

mod layout_iter;
pub use self::layout_iter::LayoutIter;

mod layout_line;
pub use self::layout_line::LayoutLine;

mod matrix;
pub use self::matrix::Matrix;

mod tab_array;
pub use self::tab_array::TabArray;

mod enums;
pub use self::enums::Alignment;
pub use self::enums::AttrType;
pub use self::enums::BidiType;
pub use self::enums::CoverageLevel;
pub use self::enums::Direction;
pub use self::enums::EllipsizeMode;
pub use self::enums::Gravity;
pub use self::enums::GravityHint;
pub use self::enums::RenderPart;
pub use self::enums::Script;
pub use self::enums::Stretch;
pub use self::enums::Style;
pub use self::enums::TabAlign;
pub use self::enums::Underline;
pub use self::enums::Variant;
pub use self::enums::Weight;
pub use self::enums::WrapMode;

mod flags;
pub use self::flags::FontMask;
pub use self::flags::FONT_MASK_FAMILY;
pub use self::flags::FONT_MASK_STYLE;
pub use self::flags::FONT_MASK_VARIANT;
pub use self::flags::FONT_MASK_WEIGHT;
pub use self::flags::FONT_MASK_STRETCH;
pub use self::flags::FONT_MASK_SIZE;
pub use self::flags::FONT_MASK_GRAVITY;

mod alias;
pub use self::alias::Glyph;
pub use self::alias::LayoutRun;

pub mod functions;

#[doc(hidden)]
pub mod traits {
    pub use super::ContextExt;
    pub use super::FontExt;
    pub use super::FontFaceExt;
    pub use super::FontFamilyExt;
    pub use super::FontMapExt;
    pub use super::FontsetExt;
    pub use super::LayoutExt;
    pub use super::RendererExt;
}
