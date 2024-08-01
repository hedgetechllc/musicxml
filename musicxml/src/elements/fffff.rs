use alloc::string::String;
use musicxml_internal::*;
use musicxml_macros::*;

/// The [Fffff] element represents an fffff dynamic marking.
///
/// ![fffff](https://hedgetechllc.github.io/musicxml/musicxml/elements/fffff.png)
#[derive(Debug, PartialEq, Eq, ElementDeserialize, ElementSerialize)]
pub struct Fffff {
  /// Element-specific attributes
  pub attributes: (),
  /// Element-specific content
  pub content: (),
}
