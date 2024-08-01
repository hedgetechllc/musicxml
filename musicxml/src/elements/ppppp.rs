use alloc::string::String;
use musicxml_internal::*;
use musicxml_macros::*;

/// The [Ppppp] element represents a pppp dynamic marking.
///
/// ![ppppp](https://hedgetechllc.github.io/musicxml/musicxml/elements/ppppp.png)
#[derive(Debug, PartialEq, Eq, ElementDeserialize, ElementSerialize)]
pub struct Ppppp {
  /// Element-specific attributes
  pub attributes: (),
  /// Element-specific content
  pub content: (),
}
