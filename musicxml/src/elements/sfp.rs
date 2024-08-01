use alloc::string::String;
use musicxml_internal::*;
use musicxml_macros::*;

/// The [Sfp] element represents a sforzando piano sfp dynamic marking.
///
/// ![sfp](https://hedgetechllc.github.io/musicxml/musicxml/elements/sfp.png)
#[derive(Debug, PartialEq, Eq, ElementDeserialize, ElementSerialize)]
pub struct Sfp {
  /// Element-specific attributes
  pub attributes: (),
  /// Element-specific content
  pub content: (),
}
