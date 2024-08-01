use alloc::string::String;
use musicxml_internal::*;
use musicxml_macros::*;

/// The [N] element represents a niente dynamic marking.
///
/// ![n](https://hedgetechllc.github.io/musicxml/musicxml/elements/n.png)
#[derive(Debug, PartialEq, Eq, ElementDeserialize, ElementSerialize)]
pub struct N {
  /// Element-specific attributes
  pub attributes: (),
  /// Element-specific content
  pub content: (),
}
