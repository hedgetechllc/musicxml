use crate::datatypes::ClefSign;
use musicxml_internal::*;
use musicxml_macros::*;

/// The [Sign] element represents the clef symbol.
#[derive(Debug, PartialEq, Eq, ElementDeserialize, ElementSerialize)]
pub struct Sign {
  /// Element-specific attributes
  pub attributes: (),
  /// Element-specific content
  pub content: ClefSign,
}
