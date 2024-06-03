use musicxml_internal::*;
use musicxml_macros::*;

/// The [Fifths] element represents the number of flats or sharps in a traditional key signature.
///
/// Negative numbers are used for flats and positive numbers for sharps, reflecting the key's placement within the circle of fifths (hence the element name).
#[derive(Debug, PartialEq, Eq, ElementDeserialize, ElementSerialize)]
pub struct Fifths {
  /// Element-specific attributes
  pub attributes: (),
  /// Element-specific content
  pub content: crate::datatypes::Fifths,
}
