use crate::datatypes::{Color, FontFamily, FontSize, FontStyle, FontWeight, Id, NumberLevel, PedalType, Tenths, YesNo};
use musicxml_internal::*;
use musicxml_macros::*;

/// Attributes pertaining to the [Pedal] element.
#[derive(Debug, PartialEq, Eq, AttributeDeserialize, AttributeSerialize)]
pub struct PedalAttributes {
  /// Distinguishes different types of pedal directions.
  pub r#type: PedalType,
  /// Used only when the `sign` attribute is yes and the `type` is start or sostenuto; otherwise it is ignored.
  /// If yes, the short P and S signs are used. If no, the full Ped and Sost signs are used. It is no if not specified.
  pub abbreviated: Option<YesNo>,
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
  /// If yes, then pedal lines are used.
  pub line: Option<YesNo>,
  /// Distinguishes multiple pedals when they overlap in MusicXML document order.
  pub number: Option<NumberLevel>,
  /// Changes the horizontal position relative to the default position, either as computed by the individual program, or as overridden by the `default_x` attribute.
  /// Positive x is right and negative x is left. It should be interpreted in the context of the [Offset][super::Offset] element or directive attribute if those are present.
  pub relative_x: Option<Tenths>,
  /// Changes the vertical position relative to the default position, either as computed by the individual program, or as overridden by the `default_y` attribute.
  /// Positive y is up and negative y is down. It should be interpreted in the context of the `placement` attribute if that is present.
  pub relative_y: Option<Tenths>,
  /// If yes, then Ped, Sost, and * signs are used. For compatibility with older versions, it is yes if not specified if the `line` attribute is no,
  /// and is no if not specified if the line attribute is yes. If no, the `alignment` attributes are ignored.
  pub sign: Option<YesNo>,
}

/// The [Pedal] element represents piano pedal marks, including damper and sostenuto pedal marks.
///
/// ![Pedal](pedal.png)
///
/// The soft pedal is not included here because there is no special symbol or graphic used for it beyond what can be specified
/// with [Words][super::Words] and [Bracket][super::Bracket] elements.
#[derive(Debug, PartialEq, Eq, ElementDeserialize, ElementSerialize)]
pub struct Pedal {
  /// Element-specific attributes
  pub attributes: PedalAttributes,
  /// Element-specific content
  pub content: (),
}
