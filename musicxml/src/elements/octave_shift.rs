use crate::datatypes::{
  Color, FontFamily, FontSize, FontStyle, FontWeight, Id, NumberLevel, PositiveInteger, Tenths, UpDownStopContinue,
};
use musicxml_internal::*;
use musicxml_macros::*;

/// Attributes pertaining to the [OctaveShift] element.
#[derive(Debug, PartialEq, Eq, AttributeDeserialize, AttributeSerialize)]
pub struct OctaveShiftAttributes {
  /// Indicates if this is the start, stop, or continuation of the octave shift.
  /// The start is specified as a shift up or down from their performed values.
  pub r#type: UpDownStopContinue,
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
  /// A comma-separated list of font names.
  pub font_family: Option<FontFamily>,
  /// One of the CSS sizes or a numeric point size.
  pub font_size: Option<FontSize>,
  /// Normal or italic style.
  pub font_style: Option<FontStyle>,
  /// Normal or bold weight.
  pub font_weight: Option<FontWeight>,
  /// Specifies an ID that is unique to the entire document.
  pub id: Option<Id>,
  /// Distinguishes multiple octave shifts when they overlap in MusicXML document order.
  pub number: Option<NumberLevel>,
  /// Changes the horizontal position relative to the default position, either as computed by the individual program, or as overridden by the `default_x` attribute.
  /// Positive x is right and negative x is left. It should be interpreted in the context of the [Offset][super::Offset] element or directive attribute if those are present.
  pub relative_x: Option<Tenths>,
  /// Changes the vertical position relative to the default position, either as computed by the individual program, or as overridden by the `default_y` attribute.
  /// Positive y is up and negative y is down. It should be interpreted in the context of the `placement` attribute if that is present.
  pub relative_y: Option<Tenths>,
  /// Specifies the symbol size to use for an editorial indication. If not specified, it is left to application defaults.
  pub size: Option<PositiveInteger>,
  /// The length of spaces in a dashed line.
  pub space_length: Option<Tenths>,
}

impl Default for OctaveShiftAttributes {
  fn default() -> Self {
    OctaveShiftAttributes {
      r#type: UpDownStopContinue::Up,
      color: None,
      dash_length: None,
      default_x: None,
      default_y: None,
      font_family: None,
      font_size: None,
      font_style: None,
      font_weight: None,
      id: None,
      number: None,
      relative_x: None,
      relative_y: None,
      size: None,
      space_length: None,
    }
  }
}

/// The [OctaveShift] element indicates where notes are shifted up or down from their performed values because of printing difficulty.
///
/// ![OctaveShift](octave-shift.png)
///
/// A treble clef line noted with 8va will be indicated with an [OctaveShift] down from the pitch data indicated in the notes.
#[derive(Debug, PartialEq, Eq, ElementDeserialize, ElementSerialize)]
#[rename("octave-shift")]
pub struct OctaveShift {
  /// Element-specific attributes
  pub attributes: OctaveShiftAttributes,
  /// Element-specific content
  pub content: (),
}
