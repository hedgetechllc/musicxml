use musicxml_internal::*;
use musicxml_macros::*;

/// The [Ffff] element represents an ffff dynamic marking.
///
/// ![ffff](ffff.png)
#[derive(Debug, PartialEq, Eq, ElementDeserialize, ElementSerialize)]
pub struct Ffff {
  /// Element-specific attributes
  pub attributes: (),
  /// Element-specific content
  pub content: (),
}
