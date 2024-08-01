use crate::datatypes::{
  Color, EnclosureShape, FontFamily, FontSize, FontStyle, FontWeight, Id, LeftCenterRight, NumberOfLines,
  NumberOrNormal, RotationDegrees, Tenths, TextDirection, Valign, XmlLang, XmlSpace,
};
use alloc::{string::String, vec::Vec};
use musicxml_internal::*;
use musicxml_macros::*;

/// Attributes pertaining to the [Words] element.
#[derive(Debug, Default, PartialEq, Eq, AttributeDeserialize, AttributeSerialize)]
pub struct WordsAttributes {
  /// Indicates the color of an element.
  pub color: Option<Color>,
  /// Changes the computation of the default horizontal position.
  /// The origin is changed relative to the left-hand side of the note or the musical position within the bar.
  /// Positive x is right and negative x is left.
  ///
  /// This attribute provides higher-resolution positioning data than the [Offset][super::Offset] element.
  /// Applications reading a MusicXML file that can understand both features should generally rely on this attribute for its greater accuracy.
  pub default_x: Option<Tenths>,
  /// Changes the computation of the default vertical position.
  /// The origin is changed relative to the top line of the staff. Positive y is up and negative y is down.
  ///
  /// This attribute provides higher-resolution positioning data than the `placement` attribute.
  /// Applications reading a MusicXML file that can understand both attributes should generally rely on this attribute for its greater accuracy.
  pub default_y: Option<Tenths>,
  /// Adjusts and overrides the Unicode bidirectional text algorithm, similar to the Directionality data category in the
  /// [W3C Internationalization Tag Set recommendation](https://www.w3.org/TR/2007/REC-its-20070403/#directionality).
  /// The default value is ltr. This attribute is typically used by applications that store text in left-to-right visual order rather than logical order.
  /// Such applications can use the lro value to better communicate with other applications that more fully support bidirectional text.
  pub dir: Option<TextDirection>,
  /// Formatting of an enclosure around text or symbols.
  pub enclosure: Option<EnclosureShape>,
  /// A comma-separated list of font names.
  pub font_family: Option<FontFamily>,
  /// One of the CSS sizes or a numeric point size.
  pub font_size: Option<FontSize>,
  /// Normal or italic style.
  pub font_style: Option<FontStyle>,
  /// Normal or bold weight.
  pub font_weight: Option<FontWeight>,
  /// In cases where text extends over more than one line, horizontal alignment and justify values can be different.
  /// The most typical case is for credits, such as:
  ///
  /// ```text
  /// Words and music by
  ///   Pat Songwriter
  /// ```
  /// Typically this type of credit is aligned to the right, so that the position information refers to the right-most part of the text.
  /// But in this example, the text is center-justified, not right-justified.
  ///
  /// The `halign` attribute is used in these situations. If it is not present, its value is the same as for the `justify` attribute.
  /// For elements where a justify attribute is not allowed, the default is implementation-dependent.
  pub halign: Option<LeftCenterRight>,
  /// Specifies an ID that is unique to the entire document.
  pub id: Option<Id>,
  /// Indicates left, center, or right justification. The default value varies for different elements.
  /// For elements where the `justify` attribute is present but the `halign` attribute is not,
  /// the `justify` attribute indicates horizontal alignment as well as justification.
  pub justify: Option<LeftCenterRight>,
  /// Specifies text tracking. Values are either normal, which allows flexibility of letter-spacing for purposes of text justification,
  /// or a number representing the number of ems to add between each letter. The number may be negative in order to subtract space.
  /// The value is normal if not specified.
  pub letter_spacing: Option<NumberOrNormal>,
  /// Specifies text leading. Values are either normal or a number representing the percentage of the current font height to use for leading.
  /// It is normal if not specified. The exact normal value is implementation-dependent, but values between 100 and 120 are recommended.
  pub line_height: Option<NumberOrNormal>,
  /// Number of lines to use when striking through text.
  pub line_through: Option<NumberOfLines>,
  /// Number of lines to use when overlining text.
  pub overline: Option<NumberOfLines>,
  /// Changes the horizontal position relative to the default position, either as computed by the individual program, or as overridden by the `default_x` attribute.
  /// Positive x is right and negative x is left. It should be interpreted in the context of the [Offset][super::Offset] element or directive attribute if those are present.
  pub relative_x: Option<Tenths>,
  /// Changes the vertical position relative to the default position, either as computed by the individual program, or as overridden by the `default_y` attribute.
  /// Positive y is up and negative y is down. It should be interpreted in the context of the `placement` attribute if that is present.
  pub relative_y: Option<Tenths>,
  /// Used to rotate text around the alignment point specified by the `halign` and `valign` attributes.
  /// Positive values are clockwise rotations, while negative values are counter-clockwise rotations.
  pub rotation: Option<RotationDegrees>,
  /// Number of lines to use when underlining text.
  pub underline: Option<NumberOfLines>,
  /// Indicates vertical alignment to the top, middle, bottom, or baseline of the text. The default is implementation-dependent.
  pub valign: Option<Valign>,
  /// Specifies the language used in the element content. It is Italian ("it") if not specified.
  pub xml_lang: Option<XmlLang>,
  /// Indicates whether white space should be preserved by applications.
  pub xml_space: Option<XmlSpace>,
}

/// The [Words] element specifies a standard text direction.
///
/// The `enclosure` attribute is none if not specified. Left justification is used if not specified.
#[derive(Debug, PartialEq, Eq, ElementDeserialize, ElementSerialize)]
pub struct Words {
  /// Element-specific attributes
  pub attributes: WordsAttributes,
  /// Element-specific content
  pub content: String,
}
