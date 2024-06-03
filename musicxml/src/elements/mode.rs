use musicxml_internal::*;
use musicxml_macros::*;

/// The [Mode] element is used to specify major/minor and other mode distinctions.
///
/// Valid mode values include major, minor, dorian, phrygian, lydian, mixolydian, aeolian, ionian, locrian, and none.
#[derive(Debug, PartialEq, Eq, ElementDeserialize, ElementSerialize)]
pub struct Mode {
  /// Element-specific attributes
  pub attributes: (),
  /// Element-specific content
  pub content: crate::datatypes::Mode,
}
