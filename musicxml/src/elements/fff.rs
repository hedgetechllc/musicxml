use musicxml_internal::*;
use musicxml_macros::*;

/// The [Fff] element represents a triple forte dynamic marking.
///
/// ![fff](https://hedgetechllc.github.io/musicxml/musicxml/elements/fff.png)
#[derive(Debug, PartialEq, Eq, ElementDeserialize, ElementSerialize)]
pub struct Fff {
  /// Element-specific attributes
  pub attributes: (),
  /// Element-specific content
  pub content: (),
}
