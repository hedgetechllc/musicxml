use musicxml_internal::*;
use musicxml_macros::*;

/// The [Sf] element represents a sforzando sf dynamic marking.
///
/// ![sf](sf.png)
#[derive(Debug, PartialEq, Eq, ElementDeserialize, ElementSerialize)]
pub struct Sf {
  /// Element-specific attributes
  pub attributes: (),
  /// Element-specific content
  pub content: (),
}