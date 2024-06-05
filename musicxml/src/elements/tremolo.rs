use crate::datatypes::{
  AboveBelow, Color, FontFamily, FontSize, FontStyle, FontWeight, SmuflGlyphName, Tenths, TremoloMarks, TremoloType,
};
use musicxml_internal::*;
use musicxml_macros::*;

/// Attributes pertaining to the [Tremolo] element.
#[derive(Debug, Default, PartialEq, Eq, AttributeDeserialize, AttributeSerialize)]
pub struct TremoloAttributes {
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
  /// Indicates whether something is above or below another element, such as a note or a notation.
  pub placement: Option<AboveBelow>,
  /// Changes the horizontal position relative to the default position, either as computed by the individual program, or as overridden by the `default_x` attribute.
  /// Positive x is right and negative x is left. It should be interpreted in the context of the [Offset][super::Offset] element or directive attribute if those are present.
  pub relative_x: Option<Tenths>,
  /// Changes the vertical position relative to the default position, either as computed by the individual program, or as overridden by the `default_y` attribute.
  /// Positive y is up and negative y is down. It should be interpreted in the context of the `placement` attribute if that is present.
  pub relative_y: Option<Tenths>,
  /// Indicates a particular Standard Music Font Layout (SMuFL) character using its canonical glyph name.
  /// Sometimes this is a formatting choice, and sometimes this is a refinement of the semantic meaning of an element.
  pub smufl: Option<SmuflGlyphName>,
  /// Single-note tremolos use single, double-note tremolos use start or stop, and unmeasured tremolos use unmeasured. The default value is single for compatibility with Version 1.1.
  pub r#type: Option<TremoloType>,
}

/// The [Tremolo] element can be used to indicate single-note, double-note, or unmeasured tremolos.
///
/// ![Tremolo](https://hedgetechllc.github.io/musicxml/musicxml/elements/tremolo.png)
///
/// The text of the element indicates the number of tremolo marks and is an integer from 0 to 8. Note that the number of attached beams is not
/// included in this value, but is represented separately using the [Beam][super::Beam]. The value should be 0 for unmeasured tremolos.
///
/// When using double-note tremolos, the duration of each note in the tremolo should correspond to half of the notated type value. A [TimeModification][super::TimeModification] element
/// should also be added with an [ActualNotes][super::ActualNotes] value of 2 and a [NormalNotes][super::NormalNotes] value of 1. If used within a tuplet, this 2/1 ratio should be multiplied
/// by the existing tuplet ratio.
///
/// The `smufl` attribute specifies the glyph to use from the Standard Music Font Layout (SMuFL) [Tremolos](https://www.w3.org/2021/03/smufl14/tables/tremolos.html)
/// range for an unmeasured tremolo. It is ignored for other tremolo types. The SMuFL "buzzRoll" glyph is used if the attribute is missing.
///
/// Using repeater beams for indicating tremolos is deprecated as of Version 3.0.
#[derive(Debug, PartialEq, Eq, ElementDeserialize, ElementSerialize)]
pub struct Tremolo {
  /// Element-specific attributes
  pub attributes: TremoloAttributes,
  /// Element-specific content
  pub content: TremoloMarks,
}
