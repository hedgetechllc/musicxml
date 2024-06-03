use musicxml_internal::*;
use musicxml_macros::*;

/// The [Pppp] element represents a pppp dynamic marking.
/// 
/// ![pppp](pppp.png)
#[derive(Debug, PartialEq, Eq, ElementDeserialize, ElementSerialize)]
pub struct Pppp {
  /// Element-specific attributes
  pub attributes: (),
  /// Element-specific content
  pub content: (),
}
