use crate::datatypes::{AboveBelow, Color, Id, NumberLevel, Tenths, TopBottom};
use musicxml_internal::*;
use musicxml_macros::*;

/// Attributes pertaining to the [NonArpeggiate] element.
#[derive(Debug, PartialEq, Eq, AttributeDeserialize, AttributeSerialize)]
pub struct NonArpeggiateAttributes {
  /// Indicates whether this is the top or bottom of the symbol.
  pub r#type: TopBottom,
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
  /// Specifies an ID that is unique to the entire document.
  pub id: Option<Id>,
  /// Used to distinguish between two simultaneous chords bracketed separately (different numbers) or together (same number).
  pub number: Option<NumberLevel>,
  /// Indicates whether something is above or below another element, such as a note or a notation.
  pub placement: Option<AboveBelow>,
  /// Changes the horizontal position relative to the default position, either as computed by the individual program, or as overridden by the `default_x` attribute.
  /// Positive x is right and negative x is left. It should be interpreted in the context of the [Offset][super::Offset] element or directive attribute if those are present.
  pub relative_x: Option<Tenths>,
  /// Changes the vertical position relative to the default position, either as computed by the individual program, or as overridden by the `default_y` attribute.
  /// Positive y is up and negative y is down. It should be interpreted in the context of the `placement` attribute if that is present.
  pub relative_y: Option<Tenths>,
}

/// The [NonArpeggiate] element indicates that this [Note][super::Note] is at the top or bottom of a bracket indicating to not arpeggiate these notes.
///
/// Since this does not involve playback, it is only used on the top or bottom notes, not on each [Note][super::Note] as for the [Arpeggiate][super::Arpeggiate] element.
#[derive(Debug, PartialEq, Eq, ElementDeserialize, ElementSerialize)]
#[rename("non-arpeggiate")]
pub struct NonArpeggiate {
  /// Element-specific attributes
  pub attributes: NonArpeggiateAttributes,
  /// Element-specific content
  pub content: (),
}
