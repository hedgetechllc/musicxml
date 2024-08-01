use alloc::string::String;
use musicxml_internal::*;
use musicxml_macros::*;

/// The [Sfzp] element represents a sforzando piano sfzp dynamic marking.
///
/// ![sfzp](https://hedgetechllc.github.io/musicxml/musicxml/elements/sfzp.png)
#[derive(Debug, PartialEq, Eq, ElementDeserialize, ElementSerialize)]
pub struct Sfzp {
  /// Element-specific attributes
  pub attributes: (),
  /// Element-specific content
  pub content: (),
}
