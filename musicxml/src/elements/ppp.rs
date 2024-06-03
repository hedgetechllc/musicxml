use musicxml_internal::*;
use musicxml_macros::*;

/// The [Ppp] element represents a triple piano dynamic marking.
/// 
/// ![ppp](ppp.png)
#[derive(Debug, PartialEq, Eq, ElementDeserialize, ElementSerialize)]
pub struct Ppp {
  /// Element-specific attributes
  pub attributes: (),
  /// Element-specific content
  pub content: (),
}
