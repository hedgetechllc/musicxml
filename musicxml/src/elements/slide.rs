use crate::datatypes::{
  Color, FontFamily, FontSize, FontStyle, FontWeight, Id, LineType, NumberLevel, Percent, StartStop, Tenths,
  TrillBeats, YesNo,
};
use musicxml_internal::*;
use musicxml_macros::*;

/// Attributes pertaining to the [Slide] element.
#[derive(Debug, PartialEq, Eq, AttributeDeserialize, AttributeSerialize)]
pub struct SlideAttributes {
  /// Indicates if this is the start or stop of the slide.
  pub r#type: StartStop,
  /// Does the bend accelerate during playback? Default is "no".
  pub accelerate: Option<YesNo>,
  /// The number of discrete elements (like MIDI pitch bends) used to represent a continuous bend or slide. Default is 4.
  pub beats: Option<TrillBeats>,
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
  /// The percentage of the duration for starting a bend. Default is 25.
  pub first_beat: Option<Percent>,
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
  /// The percentage of the duration for ending a bend. Default is 75.
  pub last_beat: Option<Percent>,
  /// Specifies if the line is solid, dashed, dotted, or wavy.
  pub line_type: Option<LineType>,
  /// Distinguishes multiple slides when they overlap in MusicXML document order. The default value is 1.
  pub number: Option<NumberLevel>,
  /// Changes the horizontal position relative to the default position, either as computed by the individual program, or as overridden by the `default_x` attribute.
  /// Positive x is right and negative x is left. It should be interpreted in the context of the [Offset][super::Offset] element or directive attribute if those are present.
  pub relative_x: Option<Tenths>,
  /// Changes the vertical position relative to the default position, either as computed by the individual program, or as overridden by the `default_y` attribute.
  /// Positive y is up and negative y is down. It should be interpreted in the context of the `placement` attribute if that is present.
  pub relative_y: Option<Tenths>,
  /// The length of spaces in a dashed line. Ignored if the corresponding `line_type` attribute is not dashed.
  pub space_length: Option<Tenths>,
}

/// The [Glissando][super::Glissando] and [Slide] elements both indicate rapidly moving from one pitch to the other so that individual notes are not discerned.
///
/// ![Slide](https://hedgetechllc.github.io/musicxml/musicxml/elements/slide.png)
///
/// A [Slide] is continuous between the two pitches and defaults to a solid line. The optional text is printed alongside the line.
#[derive(Debug, PartialEq, Eq, ElementDeserialize, ElementSerialize)]
pub struct Slide {
  /// Element-specific attributes
  pub attributes: SlideAttributes,
  /// Element-specific content
  pub content: String,
}
