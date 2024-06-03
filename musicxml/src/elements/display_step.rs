use crate::datatypes::Step;
use musicxml_internal::*;
use musicxml_macros::*;

/// The [DisplayStep] and [DisplayOctave][super::DisplayOctave] elements are used to place [Rest][super::Rest] and [Unpitched][super::Unpitched] elements
/// on the staff without implying that these elements have pitch.
/// 
/// Positioning follows the current clef. If percussion clef is used, the [DisplayStep] and [DisplayOctave][super::DisplayOctave] elements are interpreted
/// as if in treble clef, with a G in octave 4 on line 2.
#[derive(Debug, PartialEq, Eq, ElementDeserialize, ElementSerialize)]
#[rename("display-step")]
pub struct DisplayStep {
  /// Element-specific attributes
  pub attributes: (),
  /// Element-specific content
  pub content: Step,
}
