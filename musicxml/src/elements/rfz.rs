use musicxml_internal::*;
use musicxml_macros::*;

/// The [Rfz] element represents a rinforzando rfz dynamic marking.
/// 
/// ![rfz](rfz.png)
#[derive(Debug, PartialEq, Eq, ElementDeserialize, ElementSerialize)]
pub struct Rfz {
  /// Element-specific attributes
  pub attributes: (),
  /// Element-specific content
  pub content: (),
}
