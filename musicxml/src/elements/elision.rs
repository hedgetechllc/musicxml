use crate::datatypes::{Color, FontFamily, FontSize, FontStyle, FontWeight, SmuflLyricsGlyphName};
use alloc::{string::String, vec::Vec};
use musicxml_internal::*;
use musicxml_macros::*;

/// Attributes pertaining to the [Elision] element.
#[derive(Debug, Default, PartialEq, Eq, AttributeDeserialize, AttributeSerialize)]
pub struct ElisionAttributes {
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
  /// Used to specify the elision symbol to use if the element text content is empty. It is ignored otherwise.
  pub smufl: Option<SmuflLyricsGlyphName>,
}

/// The [Elision] element represents an elision between lyric syllables.
///
/// The text content specifies the symbol used to display the elision. Common values are a no-break space (Unicode 00A0), an underscore (Unicode 005F),
/// or an undertie (Unicode 203F). If the text content is empty, the `smufl` attribute is used to specify the symbol to use.
/// If neither text content nor a `smufl` attribute are present, the elision glyph is application-specific.
#[derive(Debug, PartialEq, Eq, ElementDeserialize, ElementSerialize)]
pub struct Elision {
  /// Element-specific attributes
  pub attributes: ElisionAttributes,
  /// Element-specific content
  pub content: String,
}
