use musicxml_internal::*;
use musicxml_macros::*;

/// The [Mf] element represents a mezzo forte dynamic marking.
///
/// ![mf](https://hedgetechllc.github.io/musicxml/musicxml/elements/mf.png)
#[derive(Debug, PartialEq, Eq, ElementDeserialize, ElementSerialize)]
pub struct Mf {
  /// Element-specific attributes
  pub attributes: (),
  /// Element-specific content
  pub content: (),
}
