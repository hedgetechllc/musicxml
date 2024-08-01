use alloc::string::String;
use musicxml_internal::*;
use musicxml_macros::*;

/// The [CircularArrow] element represents a circular arrow, using Unicode arrow terminology to specify the arrow direction.
///
/// ![CircularArrow](https://hedgetechllc.github.io/musicxml/musicxml/elements/circular-arrow.png)
#[derive(Debug, PartialEq, Eq, ElementDeserialize, ElementSerialize)]
#[rename("circular-arrow")]
pub struct CircularArrow {
  /// Element-specific attributes
  pub attributes: (),
  /// Element-specific content
  pub content: crate::datatypes::CircularArrow,
}
