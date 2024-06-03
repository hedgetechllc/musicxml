use super::{BendAlter, PreBend, Release, WithBar};
use crate::datatypes::{
  BendShape, Color, FontFamily, FontSize, FontStyle, FontWeight, Percent, Tenths, TrillBeats, YesNo,
};
use musicxml_internal::*;
use musicxml_macros::*;

/// Attributes pertaining to the [Bend] element.
#[derive(Debug, Default, PartialEq, Eq, AttributeDeserialize, AttributeSerialize)]
pub struct BendAttributes {
  /// Does the bend accelerate during playback? Default is "no".
  pub accelerate: Option<YesNo>,
  /// The number of discrete elements (like MIDI pitch bends) used to represent a continuous bend or slide. Default is 4.
  pub beats: Option<TrillBeats>,
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
  /// The percentage of the duration for ending a bend. Default is 75.
  pub last_beat: Option<Percent>,
  /// Changes the horizontal position relative to the default position, either as computed by the individual program, or as overridden by the `default_x` attribute.
  /// Positive x is right and negative x is left. It should be interpreted in the context of the [Offset][super::Offset] element or directive attribute if those are present.
  pub relative_x: Option<Tenths>,
  /// Changes the vertical position relative to the default position, either as computed by the individual program, or as overridden by the `default_y` attribute.
  /// Positive y is up and negative y is down. It should be interpreted in the context of the `placement` attribute if that is present.
  pub relative_y: Option<Tenths>,
  /// Distinguishes between the angled bend symbols commonly used in standard notation and the curved bend symbols commonly used in both tablature and standard notation.
  pub shape: Option<BendShape>,
}

/// Contents of the [Bend] element.
#[derive(Debug, PartialEq, Eq, ContentDeserialize, ContentSerialize)]
pub struct BendContents {
  /// The [BendAlter] element specifies the alteration of the bend.
  pub bend_alter: BendAlter,
  /// The [PreBend] element is used in guitar notation and tablature.
  pub pre_bend: Option<PreBend>,
  /// The [Release] element is used in guitar notation and tablature.
  pub release: Option<Release>,
  /// The [WithBar] element is used in guitar notation and tablature.
  pub with_bar: Option<WithBar>,
}

/// The [Bend] element is used in guitar notation and tablature.
///
/// A single note with a bend and release will contain two [Bend] elements: the first to represent the bend and the second to represent the release.
#[derive(Debug, PartialEq, Eq, ElementDeserialize, ElementSerialize)]
pub struct Bend {
  /// Element-specific attributes
  pub attributes: BendAttributes,
  #[flatten]
  /// Element-specific content
  pub content: BendContents,
}
