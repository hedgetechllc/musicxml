use alloc::string::String;
use musicxml_internal::*;
use musicxml_macros::*;

/// The [ArrowDirection] element represents the direction in which an arrow points, using Unicode arrow terminology.
#[derive(Debug, PartialEq, Eq, ElementDeserialize, ElementSerialize)]
#[rename("arrow-direction")]
pub struct ArrowDirection {
  /// Element-specific attributes
  pub attributes: (),
  /// Element-specific content
  pub content: crate::datatypes::ArrowDirection,
}
