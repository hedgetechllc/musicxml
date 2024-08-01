use super::MeasureDistance;
use alloc::{string::String, vec::Vec};
use musicxml_internal::*;
use musicxml_macros::*;

/// Contents of the [MeasureLayout] element.
#[derive(Debug, PartialEq, Eq, ContentDeserialize, ContentSerialize)]
pub struct MeasureLayoutContents {
  /// The [MeasureDistance] element includes the horizontal distance from the previous measure.
  pub measure_distance: Option<MeasureDistance>,
}

/// The [MeasureLayout] element includes the horizontal distance from the previous measure.
///
/// It applies to the current measure only.
#[derive(Debug, PartialEq, Eq, ElementDeserialize, ElementSerialize)]
#[rename("measure-layout")]
pub struct MeasureLayout {
  /// Element-specific attributes
  pub attributes: (),
  #[flatten]
  /// Element-specific content
  pub content: MeasureLayoutContents,
}
