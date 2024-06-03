use musicxml_internal::*;
use musicxml_macros::*;

/// The [Sfpp] element represents a sforzando pianissimo sfpp dynamic marking.
///
/// ![sfpp](sfpp.png)
#[derive(Debug, PartialEq, Eq, ElementDeserialize, ElementSerialize)]
pub struct Sfpp {
  /// Element-specific attributes
  pub attributes: (),
  /// Element-specific content
  pub content: (),
}
