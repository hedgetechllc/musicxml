use musicxml_internal::*;
use musicxml_macros::*;

/// The [Pp] element represents a pianissimo dynamic marking.
///
/// ![pp](pp.png)
#[derive(Debug, PartialEq, Eq, ElementDeserialize, ElementSerialize)]
pub struct Pp {
  /// Element-specific attributes
  pub attributes: (),
  /// Element-specific content
  pub content: (),
}
