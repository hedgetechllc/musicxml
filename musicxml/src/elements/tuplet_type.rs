use crate::datatypes::{Color, FontFamily, FontSize, FontStyle, FontWeight, NoteTypeValue};
use musicxml_internal::*;
use musicxml_macros::*;

/// Attributes pertaining to the [TupletType] element.
#[derive(Debug, Default, PartialEq, Eq, AttributeDeserialize, AttributeSerialize)]
pub struct TupletTypeAttributes {
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

/// The [TupletType] element indicates the graphical note type of the notes for this portion of the tuplet.
#[derive(Debug, PartialEq, Eq, ElementDeserialize, ElementSerialize)]
#[rename("tuplet-type")]
pub struct TupletType {
  /// Element-specific attributes
  pub attributes: TupletTypeAttributes,
  /// Element-specific content
  pub content: NoteTypeValue,
}
