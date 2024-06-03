use musicxml_internal::*;
use musicxml_macros::*;

/// The [Straight] element specifies that no swing is present, so consecutive notes have equal durations.
#[derive(Debug, PartialEq, Eq, ElementDeserialize, ElementSerialize)]
pub struct Straight {
  /// Element-specific attributes
  pub attributes: (),
  /// Element-specific content
  pub content: (),
}
