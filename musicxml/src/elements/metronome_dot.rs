use alloc::string::String;
use musicxml_internal::*;
use musicxml_macros::*;

/// The [MetronomeDot] element works like the [Dot][super::Dot] element in defining metric relationships.
#[derive(Debug, Default, PartialEq, Eq, ElementDeserialize, ElementSerialize)]
#[rename("metronome-dot")]
pub struct MetronomeDot {
  /// Element-specific attributes
  pub attributes: (),
  /// Element-specific content
  pub content: (),
}
