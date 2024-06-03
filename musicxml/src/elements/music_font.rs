use crate::datatypes::{FontFamily, FontSize, FontStyle, FontWeight};
use musicxml_internal::*;
use musicxml_macros::*;

/// Attributes pertaining to the [MusicFont] element.
#[derive(Debug, Default, PartialEq, Eq, AttributeDeserialize, AttributeSerialize)]
pub struct MusicFontAttributes {
  /// A comma-separated list of font names.
  pub font_family: Option<FontFamily>,
  /// One of the CSS sizes or a numeric point size.
  pub font_size: Option<FontSize>,
  /// Normal or italic style.
  pub font_style: Option<FontStyle>,
  /// Normal or bold weight.
  pub font_weight: Option<FontWeight>,
}

/// The [MusicFont] element represents the default values for the music font in the score.
#[derive(Debug, PartialEq, Eq, ElementDeserialize, ElementSerialize)]
#[rename("music-font")]
pub struct MusicFont {
  /// Element-specific attributes
  pub attributes: MusicFontAttributes,
  /// Element-specific content
  pub content: (),
}
