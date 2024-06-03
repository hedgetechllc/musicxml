use crate::datatypes::{Color, FontFamily, FontSize, FontStyle, FontWeight, NonNegativeInteger};
use musicxml_internal::*;
use musicxml_macros::*;

/// Attributes pertaining to the [Fret] element.
#[derive(Debug, Default, PartialEq, Eq, AttributeDeserialize, AttributeSerialize)]
pub struct FretAttributes {
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

/// The [Fret] element is used with tablature notation and chord diagrams.
/// 
/// Fret numbers start with 0 for an open string and 1 for the first fret.
#[derive(Debug, PartialEq, Eq, ElementDeserialize, ElementSerialize)]
pub struct Fret {
  /// Element-specific attributes
  pub attributes: FretAttributes,
  /// Element-specific content
  pub content: NonNegativeInteger,
}
