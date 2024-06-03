use crate::datatypes::{AboveBelow, Color, Id, NumberLevel, Tenths, UpDown, YesNo};
use musicxml_internal::*;
use musicxml_macros::*;

/// Attributes pertaining to the [Arpeggiate] element.
#[derive(Debug, Default, PartialEq, Eq, AttributeDeserialize, AttributeSerialize)]
pub struct ArpeggiateAttributes {
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
  /// Used if there is an arrow on the arpeggio sign. By default, arpeggios go from the lowest to highest note.
  pub direction: Option<UpDown>,
  /// Specifies an ID that is unique to the entire document.
  pub id: Option<Id>,
  /// Used to distinguish between two simultaneous chords arpeggiated separately (different numbers) or together (same number).
  pub number: Option<NumberLevel>,
  /// Indicates whether something is above or below another element, such as a note or a notation.
  pub placement: Option<AboveBelow>,
  /// Changes the horizontal position relative to the default position, either as computed by the individual program, or as overridden by the `default_x` attribute.
  /// Positive x is right and negative x is left. It should be interpreted in the context of the [Offset][super::Offset] element or directive attribute if those are present.
  pub relative_x: Option<Tenths>,
  /// Changes the vertical position relative to the default position, either as computed by the individual program, or as overridden by the `default_y` attribute.
  /// Positive y is up and negative y is down. It should be interpreted in the context of the `placement` attribute if that is present.
  pub relative_y: Option<Tenths>,
  /// If yes, indicates that the arpeggio continues onto another staff within the part.
  /// This serves as a hint to applications and is not required for cross-staff arpeggios.
  pub unbroken: Option<YesNo>,
}

/// The [Arpeggiate] element indicates that this note is part of an arpeggiated chord.
/// 
/// ![Arpeggiate](arpeggiate.png)
/// 
/// The length of the sign can be determined from the position attributes for the [Arpeggiate] elements used with the
/// top and bottom notes of the arpeggiated chord.
#[derive(Debug, PartialEq, Eq, ElementDeserialize, ElementSerialize)]
pub struct Arpeggiate {
  /// Element-specific attributes
  pub attributes: ArpeggiateAttributes,
  /// Element-specific content
  pub content: (),
}
