use musicxml_internal::*;
use musicxml_macros::*;

/// The [OctaveChange] element indicates how many octaves to add to get from written pitch to sounding pitch.
///
/// The [OctaveChange] element should be included when using transposition intervals of an octave or more,
/// and should not be present for intervals of less than an octave.
#[derive(Debug, PartialEq, Eq, ElementDeserialize, ElementSerialize)]
#[rename("octave-change")]
pub struct OctaveChange {
  /// Element-specific attributes
  pub attributes: (),
  /// Element-specific content
  pub content: i8,
}
