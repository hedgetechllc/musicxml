use crate::datatypes::{Color, FontFamily, FontSize, FontStyle, FontWeight};
use alloc::{string::String, vec::Vec};
use musicxml_internal::*;
use musicxml_macros::*;

/// Attributes pertaining to the [TupletDot] element.
#[derive(Debug, Default, PartialEq, Eq, AttributeDeserialize, AttributeSerialize)]
pub struct TupletDotAttributes {
  /// Indicates the color of an element.
  pub color: Option<Color>,
  /// A comma-separated list of font names.
  pub font_family: Option<FontFamily>,
  /// One of the CSS sizes or a numeric point size.
  pub font_size: Option<FontSize>,
  /// Normal or italic style.
  pub font_style: Option<FontStyle>,
  /// Normal or bold weight.
  pub font_weight: Option<FontWeight>,
}

/// The [TupletDot] element is used to specify dotted tuplet types.
#[derive(Debug, PartialEq, Eq, ElementDeserialize, ElementSerialize)]
#[rename("tuplet-dot")]
pub struct TupletDot {
  /// Element-specific attributes
  pub attributes: TupletDotAttributes,
  /// Element-specific content
  pub content: (),
}
