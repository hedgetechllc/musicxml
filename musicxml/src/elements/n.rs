use musicxml_internal::*;
use musicxml_macros::*;

/// The [N] element represents a niente dynamic marking.
///
/// ![n](n.png)
#[derive(Debug, PartialEq, Eq, ElementDeserialize, ElementSerialize)]
pub struct N {
  /// Element-specific attributes
  pub attributes: (),
  /// Element-specific content
  pub content: (),
}
