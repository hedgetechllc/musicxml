use crate::datatypes::PositiveInteger;
use musicxml_internal::*;
use musicxml_macros::*;

/// The [FrameStrings] element gives the overall size of the frame in vertical lines (strings).
#[derive(Debug, PartialEq, Eq, ElementDeserialize, ElementSerialize)]
#[rename("frame-strings")]
pub struct FrameStrings {
  /// Element-specific attributes
  pub attributes: (),
  /// Element-specific content
  pub content: PositiveInteger,
}
