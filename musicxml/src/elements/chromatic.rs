use crate::datatypes::Semitones;
use musicxml_internal::*;
use musicxml_macros::*;

/// The [Chromatic] element represents the number of semitones needed to get from written to sounding pitch.
///
/// This value does not include [OctaveChange][super::OctaveChange] values; the values for both elements need to be added
/// to the written pitch to get the correct sounding pitch.
#[derive(Debug, PartialEq, Eq, ElementDeserialize, ElementSerialize)]
pub struct Chromatic {
  /// Element-specific attributes
  pub attributes: (),
  /// Element-specific content
  pub content: Semitones,
}
