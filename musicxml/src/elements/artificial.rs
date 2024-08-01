use alloc::string::String;
use musicxml_internal::*;
use musicxml_macros::*;

/// The [Artificial] element indicates that this is an artificial harmonic.
///
/// ![Artificial](https://hedgetechllc.github.io/musicxml/musicxml/elements/artificial.png)
#[derive(Debug, PartialEq, Eq, ElementDeserialize, ElementSerialize)]
pub struct Artificial {
  /// Element-specific attributes
  pub attributes: (),
  /// Element-specific content
  pub content: (),
}
