use crate::datatypes::{
  AboveBelow, Color, FontFamily, FontSize, FontStyle, FontWeight, Id, NumberLevel, SmuflGlyphName, StartStopSingle,
  Tenths, YesNo,
};
use alloc::{string::String, vec::Vec};
use musicxml_internal::*;
use musicxml_macros::*;

/// Attributes pertaining to the [OtherNotation] element.
#[derive(Debug, PartialEq, Eq, AttributeDeserialize, AttributeSerialize)]
pub struct OtherNotationAttributes {
  /// Indicates if this is a single-note notation, or the start or stop of a multi-note notation.
  pub r#type: StartStopSingle,
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
  /// Distinguishes multiple notations when they overlap in MusicXML document order. The default value is 1.
  pub number: Option<NumberLevel>,
  /// Indicates whether something is above or below another element, such as a note or a notation.
  pub placement: Option<AboveBelow>,
  /// Specifies whether or not to print an object. It is yes if not specified.
  pub print_object: Option<YesNo>,
  /// Changes the horizontal position relative to the default position, either as computed by the individual program, or as overridden by the `default_x` attribute.
  /// Positive x is right and negative x is left. It should be interpreted in the context of the [Offset][super::Offset] element or directive attribute if those are present.
  pub relative_x: Option<Tenths>,
  /// Changes the vertical position relative to the default position, either as computed by the individual program, or as overridden by the `default_y` attribute.
  /// Positive y is up and negative y is down. It should be interpreted in the context of the `placement` attribute if that is present.
  pub relative_y: Option<Tenths>,
  /// Indicates a particular Standard Music Font Layout (SMuFL) character using its canonical glyph name.
  /// Sometimes this is a formatting choice, and sometimes this is a refinement of the semantic meaning of an element.
  pub smufl: Option<SmuflGlyphName>,
}

/// The [OtherNotation] element is used to define any notations not yet in the MusicXML format.
///
/// It handles notations where more specific extension elements such as [OtherDynamics][super::OtherDynamics] and
/// [OtherTechnical][super::OtherTechnical] are not appropriate.
///
/// The `smufl` attribute can be used to specify a particular notation, allowing application interoperability without requiring every
/// Standard Music Font Layout (SMuFL) glyph to have a MusicXML element equivalent. Using the [OtherNotation] element without the `smufl` attribute
/// allows for extended representation, though without application interoperability.
#[derive(Debug, PartialEq, Eq, ElementDeserialize, ElementSerialize)]
#[rename("other-notation")]
pub struct OtherNotation {
  /// Element-specific attributes
  pub attributes: OtherNotationAttributes,
  /// Element-specific content
  pub content: String,
}
