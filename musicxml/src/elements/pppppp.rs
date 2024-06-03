use musicxml_internal::*;
use musicxml_macros::*;

/// The [Pppppp] element represents a pppp dynamic marking.
///
/// ![pppppp](pppppp.png)
#[derive(Debug, PartialEq, Eq, ElementDeserialize, ElementSerialize)]
pub struct Pppppp {
  /// Element-specific attributes
  pub attributes: (),
  /// Element-specific content
  pub content: (),
}
