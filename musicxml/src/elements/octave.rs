use musicxml_internal::*;
use musicxml_macros::*;

/// Octaves are represented by the numbers 0 to 9, where 4 indicates the octave started by middle C.
#[derive(Debug, PartialEq, Eq, ElementDeserialize, ElementSerialize)]
pub struct Octave {
  /// Element-specific attributes
  pub attributes: (),
  /// Element-specific content
  pub content: crate::datatypes::Octave,
}
