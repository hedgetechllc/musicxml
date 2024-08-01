use alloc::string::String;
use musicxml_internal::*;
use musicxml_macros::*;

/// The [P] element represents a piano dynamic marking.
///
/// ![p](https://hedgetechllc.github.io/musicxml/musicxml/elements/p.png)
#[derive(Debug, PartialEq, Eq, ElementDeserialize, ElementSerialize)]
pub struct P {
  /// Element-specific attributes
  pub attributes: (),
  /// Element-specific content
  pub content: (),
}
