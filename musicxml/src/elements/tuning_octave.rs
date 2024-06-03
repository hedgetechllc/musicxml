use crate::datatypes::Octave;
use musicxml_internal::*;
use musicxml_macros::*;

/// The [TuningOctave] element is represented like the [Octave][super::Octave] element, with a different name to reflect its different function in string tuning.
#[derive(Debug, PartialEq, Eq, ElementDeserialize, ElementSerialize)]
#[rename("tuning-octave")]
pub struct TuningOctave {
  /// Element-specific attributes
  pub attributes: (),
  /// Element-specific content
  pub content: Octave,
}
