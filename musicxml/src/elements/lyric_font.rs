use crate::datatypes::{FontFamily, FontSize, FontStyle, FontWeight, NmToken, Token};
use musicxml_internal::*;
use musicxml_macros::*;

/// Attributes pertaining to the [LyricFont] element.
#[derive(Debug, Default, PartialEq, Eq, AttributeDeserialize, AttributeSerialize)]
pub struct LyricFontAttributes {
  /// A comma-separated list of font names.
  pub font_family: Option<FontFamily>,
  /// One of the CSS sizes or a numeric point size.
  pub font_size: Option<FontSize>,
  /// Normal or italic style.
  pub font_style: Option<FontStyle>,
  /// Normal or bold weight.
  pub font_weight: Option<FontWeight>,
  /// The lyric name for which this is the default, corresponding to the `name` attribute in the [Lyric][super::Lyric] element.
  pub name: Option<Token>,
  /// The lyric number for which this is the default, corresponding to the `number` attribute in the [Lyric][super::Lyric] element.
  pub number: Option<NmToken>,
}

/// The [LyricFont] element specifies the default font for a particular name and number of lyric.
#[derive(Debug, PartialEq, Eq, ElementDeserialize, ElementSerialize)]
#[rename("lyric-font")]
pub struct LyricFont {
  /// Element-specific attributes
  pub attributes: LyricFontAttributes,
  /// Element-specific content
  pub content: (),
}
