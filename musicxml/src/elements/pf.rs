use alloc::string::String;
use musicxml_internal::*;
use musicxml_macros::*;

/// The [Pf] element represents a piano forte dynamic marking.
///
/// ![pf](https://hedgetechllc.github.io/musicxml/musicxml/elements/pf.png)
#[derive(Debug, PartialEq, Eq, ElementDeserialize, ElementSerialize)]
pub struct Pf {
  /// Element-specific attributes
  pub attributes: (),
  /// Element-specific content
  pub content: (),
}
