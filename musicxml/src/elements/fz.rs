use alloc::string::String;
use musicxml_internal::*;
use musicxml_macros::*;

/// The [Fz] element represents a forzando fz dynamic marking.
///
/// ![fz](https://hedgetechllc.github.io/musicxml/musicxml/elements/fz.png)
#[derive(Debug, PartialEq, Eq, ElementDeserialize, ElementSerialize)]
pub struct Fz {
  /// Element-specific attributes
  pub attributes: (),
  /// Element-specific content
  pub content: (),
}
