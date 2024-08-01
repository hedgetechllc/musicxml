use alloc::string::String;
use musicxml_internal::*;
use musicxml_macros::*;

/// The presence of an [Arrowhead] element indicates that only the arrowhead is displayed within the [Arrow][super::Arrow], not the arrow stem.
#[derive(Debug, PartialEq, Eq, ElementDeserialize, ElementSerialize)]
pub struct Arrowhead {
  /// Element-specific attributes
  pub attributes: (),
  /// Element-specific content
  pub content: (),
}
