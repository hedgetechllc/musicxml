use crate::datatypes::{
  AboveBelow, Color, FontFamily, FontSize, FontStyle, FontWeight, LineLength, LineShape, LineType, Tenths,
};
use musicxml_internal::*;
use musicxml_macros::*;

/// Attributes pertaining to the [Scoop] element.
#[derive(Debug, Default, PartialEq, Eq, AttributeDeserialize, AttributeSerialize)]
pub struct ScoopAttributes {
  /// Indicates the color of an element.
  pub color: Option<Color>,
  /// The length of dashes in a dashed line. Ignored if the corresponding `line_type` attribute is not dashed.
  pub dash_length: Option<Tenths>,
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
  /// A comma-separated list of font names.
  pub font_family: Option<FontFamily>,
  /// One of the CSS sizes or a numeric point size.
  pub font_size: Option<FontSize>,
  /// Normal or italic style.
  pub font_style: Option<FontStyle>,
  /// Normal or bold weight.
  pub font_weight: Option<FontWeight>,
  /// Distinguishes between different line lengths for doit, falloff, plop, and scoop articulations.
  pub line_length: Option<LineLength>,
  /// Is the line straight or curved?
  pub line_shape: Option<LineShape>,
  /// Specifies if the line is solid, dashed, dotted, or wavy.
  pub line_type: Option<LineType>,
  /// Indicates whether something is above or below another element, such as a note or a notation.
  pub placement: Option<AboveBelow>,
  /// Changes the horizontal position relative to the default position, either as computed by the individual program, or as overridden by the `default_x` attribute.
  /// Positive x is right and negative x is left. It should be interpreted in the context of the [Offset][super::Offset] element or directive attribute if those are present.
  pub relative_x: Option<Tenths>,
  /// Changes the vertical position relative to the default position, either as computed by the individual program, or as overridden by the `default_y` attribute.
  /// Positive y is up and negative y is down. It should be interpreted in the context of the `placement` attribute if that is present.
  pub relative_y: Option<Tenths>,
  /// The length of spaces in a dashed line. Ignored if the corresponding `line_type` attribute is not dashed.
  pub space_length: Option<Tenths>,
}

/// The [Scoop] element is an indeterminate slide attached to a single note.
///
/// ![Scoop](https://hedgetechllc.github.io/musicxml/musicxml/elements/scoop.png)
///
/// The scoop appears before the main note and comes from below the main pitch.
#[derive(Debug, PartialEq, Eq, ElementDeserialize, ElementSerialize)]
pub struct Scoop {
  /// Element-specific attributes
  pub attributes: ScoopAttributes,
  /// Element-specific content
  pub content: (),
}
