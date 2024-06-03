use musicxml_internal::*;
use musicxml_macros::*;

/// The [Mp] element represents a mezzo piano dynamic marking.
#[derive(Debug, PartialEq, Eq, ElementDeserialize, ElementSerialize)]
pub struct Mp {
  /// Element-specific attributes
  pub attributes: (),
  /// Element-specific content
  pub content: (),
}
