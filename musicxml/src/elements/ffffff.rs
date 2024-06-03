use musicxml_internal::*;
use musicxml_macros::*;

/// The [Ffffff] element represents an ffffff dynamic marking.
///
/// ![ffffff](ffffff.png)
#[derive(Debug, PartialEq, Eq, ElementDeserialize, ElementSerialize)]
pub struct Ffffff {
  /// Element-specific attributes
  pub attributes: (),
  /// Element-specific content
  pub content: (),
}
