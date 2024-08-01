use alloc::string::String;
use musicxml_internal::*;
use musicxml_macros::*;

/// The [Sfz] element represents a sforzando sfz dynamic marking.
///
/// ![sfz](https://hedgetechllc.github.io/musicxml/musicxml/elements/sfz.png)
#[derive(Debug, PartialEq, Eq, ElementDeserialize, ElementSerialize)]
pub struct Sfz {
  /// Element-specific attributes
  pub attributes: (),
  /// Element-specific content
  pub content: (),
}
