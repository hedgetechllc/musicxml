use alloc::string::String;
use musicxml_internal::*;
use musicxml_macros::*;

/// The [Sfpp] element represents a sforzando pianissimo sfpp dynamic marking.
///
/// ![sfpp](https://hedgetechllc.github.io/musicxml/musicxml/elements/sfpp.png)
#[derive(Debug, PartialEq, Eq, ElementDeserialize, ElementSerialize)]
pub struct Sfpp {
  /// Element-specific attributes
  pub attributes: (),
  /// Element-specific content
  pub content: (),
}
