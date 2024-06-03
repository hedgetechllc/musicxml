use crate::datatypes::PositiveInteger;
use musicxml_internal::*;
use musicxml_macros::*;

/// The [FrameFrets] element gives the overall size of the frame in horizontal spaces (frets).
#[derive(Debug, PartialEq, Eq, ElementDeserialize, ElementSerialize)]
#[rename("frame-frets")]
pub struct FrameFrets {
  /// Element-specific attributes
  pub attributes: (),
  /// Element-specific content
  pub content: PositiveInteger,
}
