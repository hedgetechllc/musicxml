use crate::datatypes::{Color, Id, LineType, NumberLevel, Tenths, WedgeType, YesNo};
use musicxml_internal::*;
use musicxml_macros::*;

/// Attributes pertaining to the [Wedge] element.
#[derive(Debug, PartialEq, Eq, AttributeDeserialize, AttributeSerialize)]
pub struct WedgeAttributes {
  /// The value is crescendo for the start of a wedge that is closed at the left side, diminuendo for the start of a wedge that is closed on the right side, and stop for the end of a wedge.
  pub r#type: WedgeType,
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
  /// Specifies an ID that is unique to the entire document.
  pub id: Option<Id>,
  /// Specifies if the line is solid, dashed, dotted, or wavy.
  pub line_type: Option<LineType>,
  /// A value is yes indicates that a circle appears at the point of the wedge, indicating a crescendo from nothing or diminuendo to nothing.
  /// It is no if not specified, and used only when the type is crescendo, or the type is stop for a wedge that began with a diminuendo type.
  pub niente: Option<YesNo>,
  /// Distinguishes multiple wedges when they overlap in MusicXML document order.
  pub number: Option<NumberLevel>,
  /// Changes the horizontal position relative to the default position, either as computed by the individual program, or as overridden by the `default_x` attribute.
  /// Positive x is right and negative x is left. It should be interpreted in the context of the [Offset][super::Offset] element or directive attribute if those are present.
  pub relative_x: Option<Tenths>,
  /// Changes the vertical position relative to the default position, either as computed by the individual program, or as overridden by the `default_y` attribute.
  /// Positive y is up and negative y is down. It should be interpreted in the context of the `placement` attribute if that is present.
  pub relative_y: Option<Tenths>,
  /// The length of spaces in a dashed line. Ignored if the corresponding `line_type` attribute is not dashed.
  pub space_length: Option<Tenths>,
  /// Indicates the gap between the top and bottom of the wedge as measured in tenths. Ignored if specified at the start of a crescendo wedge or the end of a diminuendo wedge.
  pub spread: Option<Tenths>,
}

/// The [Wedge] element represents crescendo and diminuendo wedge symbols.
///
/// The `line_type` attribute is solid if not specified.
#[derive(Debug, PartialEq, Eq, ElementDeserialize, ElementSerialize)]
pub struct Wedge {
  /// Element-specific attributes
  pub attributes: WedgeAttributes,
  /// Element-specific content
  pub content: (),
}
