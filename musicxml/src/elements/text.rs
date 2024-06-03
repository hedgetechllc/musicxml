use crate::datatypes::{
  Color, FontFamily, FontSize, FontStyle, FontWeight, NumberOfLines, NumberOrNormal, RotationDegrees, TextDirection,
  XmlLang,
};
use musicxml_internal::*;
use musicxml_macros::*;

/// Attributes pertaining to the [TextAttributes] element.
#[derive(Debug, Default, PartialEq, Eq, AttributeDeserialize, AttributeSerialize)]
pub struct TextAttributes {
  /// Indicates the color of an element.
  pub color: Option<Color>,
  /// Adjusts and overrides the Unicode bidirectional text algorithm, similar to the Directionality data category in the
  /// [W3C Internationalization Tag Set recommendation](https://www.w3.org/TR/2007/REC-its-20070403/#directionality).
  /// The default value is ltr. This attribute is typically used by applications that store text in left-to-right visual order rather than logical order.
  /// Such applications can use the lro value to better communicate with other applications that more fully support bidirectional text.
  pub dir: Option<TextDirection>,
  /// A comma-separated list of font names.
  pub font_family: Option<FontFamily>,
  /// One of the CSS sizes or a numeric point size.
  pub font_size: Option<FontSize>,
  /// Normal or italic style.
  pub font_style: Option<FontStyle>,
  /// Normal or bold weight.
  pub font_weight: Option<FontWeight>,
  /// Specifies text tracking. Values are either normal, which allows flexibility of letter-spacing for purposes of text justification,
  /// or a number representing the number of ems to add between each letter. The number may be negative in order to subtract space.
  /// The value is normal if not specified.
  pub letter_spacing: Option<NumberOrNormal>,
  /// Number of lines to use when striking through text.
  pub line_through: Option<NumberOfLines>,
  /// Number of lines to use when overlining text.
  pub overline: Option<NumberOfLines>,
  /// Used to rotate text around the alignment point specified by the `halign` and `valign` attributes.
  /// Positive values are clockwise rotations, while negative values are counter-clockwise rotations.
  pub rotation: Option<RotationDegrees>,
  /// Number of lines to use when underlining text.
  pub underline: Option<NumberOfLines>,
  /// Specifies the language used in the element content. It is Italian ("it") if not specified.
  pub xml_lang: Option<XmlLang>,
}

/// The [Text] element represents a syllable or portion of a syllable for lyric text underlay.
/// 
/// A hyphen in the element content should only be used for an actual hyphenated word.
#[derive(Debug, Default, PartialEq, Eq, ElementDeserialize, ElementSerialize)]
pub struct Text {
  /// Element-specific attributes
  pub attributes: TextAttributes,
  /// Element-specific content
  pub content: String,
}
