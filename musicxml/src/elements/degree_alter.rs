use crate::datatypes::{Color, FontFamily, FontSize, FontStyle, FontWeight, Semitones, Tenths, YesNo};
use musicxml_internal::*;
use musicxml_macros::*;

/// Attributes pertaining to the [DegreeAlter] element.
#[derive(Debug, Default, PartialEq, Eq, AttributeDeserialize, AttributeSerialize)]
pub struct DegreeAlterAttributes {
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
  /// Indicates if plus and minus symbols should be used instead of sharp and flat symbols to display the degree alteration. It is no if not specified.
  pub plus_minus: Option<YesNo>,
  /// Changes the horizontal position relative to the default position, either as computed by the individual program, or as overridden by the `default_x` attribute.
  /// Positive x is right and negative x is left. It should be interpreted in the context of the [Offset][super::Offset] element or directive attribute if those are present.
  pub relative_x: Option<Tenths>,
  /// Changes the vertical position relative to the default position, either as computed by the individual program, or as overridden by the `default_y` attribute.
  /// Positive y is up and negative y is down. It should be interpreted in the context of the `placement` attribute if that is present.
  pub relative_y: Option<Tenths>,
}

/// The [DegreeAlter] element represents the chromatic alteration for the current degree.
///
/// If the [DegreeType][super::DegreeType] value is alter or subtract, the [DegreeAlter] value is relative to the degree already
/// in the chord based on its [Kind][super::Kind] element. If the [DegreeType][super::DegreeType] value is add, the [DegreeAlter] is relative
/// to a dominant chord (major and perfect intervals except for a minor seventh).
#[derive(Debug, PartialEq, Eq, ElementDeserialize, ElementSerialize)]
#[rename("degree-alter")]
pub struct DegreeAlter {
  /// Element-specific attributes
  pub attributes: DegreeAlterAttributes,
  /// Element-specific content
  pub content: Semitones,
}
