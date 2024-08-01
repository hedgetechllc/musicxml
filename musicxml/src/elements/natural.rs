use alloc::string::String;
use musicxml_internal::*;
use musicxml_macros::*;

/// The [Natural] element indicates that this is a natural harmonic.
///
/// ![Natural](https://hedgetechllc.github.io/musicxml/musicxml/elements/natural.png)
///
/// These are usually notated at base pitch rather than sounding pitch.
#[derive(Debug, PartialEq, Eq, ElementDeserialize, ElementSerialize)]
pub struct Natural {
  /// Element-specific attributes
  pub attributes: (),
  /// Element-specific content
  pub content: (),
}
