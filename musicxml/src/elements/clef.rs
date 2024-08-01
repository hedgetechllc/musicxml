use super::{ClefOctaveChange, Line, Sign};
use crate::datatypes::{
  Color, FontFamily, FontSize, FontStyle, FontWeight, Id, StaffNumber, SymbolSize, Tenths, YesNo,
};
use alloc::{string::String, vec::Vec};
use musicxml_internal::*;
use musicxml_macros::*;

/// Attributes pertaining to the [Clef] element.
#[derive(Debug, Default, PartialEq, Eq, AttributeDeserialize, AttributeSerialize)]
pub struct ClefAttributes {
  /// Sometimes clefs are added to the staff in non-standard line positions, either to indicate cue passages,
  /// or when there are multiple clefs present simultaneously on one staff. In this situation,
  /// the `additional` attribute is set to "yes" and the line value is ignored.
  pub additional: Option<YesNo>,
  /// Sometimes clefs at the start of a measure need to appear after the barline rather than before,
  /// as for cues or for use after a repeated section. The `after_barline` attribute is set to "yes" in this situation.
  /// This attribute is ignored for mid-measure clefs.
  pub after_barline: Option<YesNo>,
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
  /// Specifies the staff number from top to bottom within the part. The value is 1 if not present.
  pub number: Option<StaffNumber>,
  /// Specifies whether or not to print an object. It is yes if not specified.
  pub print_object: Option<YesNo>,
  /// Changes the horizontal position relative to the default position, either as computed by the individual program, or as overridden by the `default_x` attribute.
  /// Positive x is right and negative x is left. It should be interpreted in the context of the [Offset][super::Offset] element or directive attribute if those are present.
  pub relative_x: Option<Tenths>,
  /// Changes the vertical position relative to the default position, either as computed by the individual program, or as overridden by the `default_y` attribute.
  /// Positive y is up and negative y is down. It should be interpreted in the context of the `placement` attribute if that is present.
  pub relative_y: Option<Tenths>,
  /// Specifies the symbol size to use for an editorial indication. If not specified, it is left to application defaults.
  pub size: Option<SymbolSize>,
}

/// Contents of the [Clef] element.
#[derive(Debug, PartialEq, Eq, ContentDeserialize, ContentSerialize)]
pub struct ClefContents {
  /// The [Sign] element represents the clef symbol.
  pub sign: Sign,
  /// The [Line] element is used to specify the line number of the staff associated with a clef.
  pub line: Option<Line>,
  /// The [ClefOctaveChange] element is used to indicate the octave change for the clef.
  pub clef_octave_change: Option<ClefOctaveChange>,
}

/// Clefs are represented by a combination of [Sign], [Line], and [ClefOctaveChange] elements.
///
/// Clefs appear at the start of each system unless the `print_object` attribute has been set to "no" or the `additional` attribute has been set to "yes".
#[derive(Debug, PartialEq, Eq, ElementDeserialize, ElementSerialize)]
pub struct Clef {
  /// Element-specific attributes
  pub attributes: ClefAttributes,
  #[flatten]
  /// Element-specific content
  pub content: ClefContents,
}
