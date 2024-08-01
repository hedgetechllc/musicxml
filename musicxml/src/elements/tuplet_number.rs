use crate::datatypes::{Color, FontFamily, FontSize, FontStyle, FontWeight, NonNegativeInteger};
use alloc::{string::String, vec::Vec};
use musicxml_internal::*;
use musicxml_macros::*;

/// Attributes pertaining to the [TupletNumber] element.
#[derive(Debug, Default, PartialEq, Eq, AttributeDeserialize, AttributeSerialize)]
pub struct TupletNumberAttributes {
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

/// The [TupletNumber] element indicates the number of notes for this portion of the tuplet.
#[derive(Debug, PartialEq, Eq, ElementDeserialize, ElementSerialize)]
#[rename("tuplet-number")]
pub struct TupletNumber {
  /// Element-specific attributes
  pub attributes: TupletNumberAttributes,
  /// Element-specific content
  pub content: NonNegativeInteger,
}
