use crate::datatypes::{Color, Id, NumberLevel, StartStopContinue, Tenths};
use musicxml_internal::*;
use musicxml_macros::*;

/// Attributes pertaining to the [Dashes] element.
#[derive(Debug, PartialEq, Eq, AttributeDeserialize, AttributeSerialize)]
pub struct DashesAttributes {
  /// Indicates if this is the start, stop, or continuation of the dashes.
  pub r#type: StartStopContinue,
  /// Indicates the color of an element.
  pub color: Option<Color>,
  /// The length of dashes in a dashed line.
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
  /// Distinguishes multiple dashes when they overlap in MusicXML document order.
  pub number: Option<NumberLevel>,
  /// Changes the horizontal position relative to the default position, either as computed by the individual program, or as overridden by the `default_x` attribute.
  /// Positive x is right and negative x is left. It should be interpreted in the context of the [Offset][super::Offset] element or directive attribute if those are present.
  pub relative_x: Option<Tenths>,
  /// Changes the vertical position relative to the default position, either as computed by the individual program, or as overridden by the `default_y` attribute.
  /// Positive y is up and negative y is down. It should be interpreted in the context of the `placement` attribute if that is present.
  pub relative_y: Option<Tenths>,
  /// The length of spaces in a dashed line.
  pub space_length: Option<Tenths>,
}

/// The [Dashes] element represents dashes, used for instance with cresc. and dim. marks.
///
/// ![Dashes](dashes.png)
#[derive(Debug, PartialEq, Eq, ElementDeserialize, ElementSerialize)]
pub struct Dashes {
  /// Element-specific attributes
  pub attributes: DashesAttributes,
  /// Element-specific content
  pub content: (),
}
