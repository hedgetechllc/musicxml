use crate::datatypes::Octave;
use musicxml_internal::*;
use musicxml_macros::*;

/// The [DisplayStep][super::DisplayStep] and [DisplayOctave] elements are used to place [Rest][super::Rest] and [Unpitched][super::Unpitched] elements
/// on the staff without implying that these elements have pitch.
///
/// Positioning follows the current clef. If percussion clef is used, the [DisplayStep][super::DisplayStep] and [DisplayOctave] elements are interpreted
/// as if in treble clef, with a G in octave 4 on line 2.
#[derive(Debug, PartialEq, Eq, ElementDeserialize, ElementSerialize)]
#[rename("display-octave")]
pub struct DisplayOctave {
  /// Element-specific attributes
  pub attributes: (),
  /// Element-specific content
  pub content: Octave,
}
