use musicxml_internal::*;
use musicxml_macros::*;

/// The [Rf] element represents a rinforzando rf dynamic marking.
///
/// ![rf](https://hedgetechllc.github.io/musicxml/musicxml/elements/rf.png)
#[derive(Debug, PartialEq, Eq, ElementDeserialize, ElementSerialize)]
pub struct Rf {
  /// Element-specific attributes
  pub attributes: (),
  /// Element-specific content
  pub content: (),
}
