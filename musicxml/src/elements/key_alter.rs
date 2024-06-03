use crate::datatypes::Semitones;
use musicxml_internal::*;
use musicxml_macros::*;

/// The [KeyAlter] element represents the alteration for a given pitch step, represented with semitones in the same manner
/// as the [Alter][super::Alter] element.
///
/// The different element names indicate the different meaning of altering notes in a scale versus altering a sounding pitch.
#[derive(Debug, PartialEq, Eq, ElementDeserialize, ElementSerialize)]
#[rename("key-alter")]
pub struct KeyAlter {
  /// Element-specific attributes
  pub attributes: (),
  /// Element-specific content
  pub content: Semitones,
}
