use musicxml_internal::*;
use musicxml_macros::*;

/// The [F] element represents a forte dynamic marking.
///
/// ![f](https://hedgetechllc.github.io/musicxml/musicxml/elements/f.png)
#[derive(Debug, PartialEq, Eq, ElementDeserialize, ElementSerialize)]
pub struct F {
  /// Element-specific attributes
  pub attributes: (),
  /// Element-specific content
  pub content: (),
}
