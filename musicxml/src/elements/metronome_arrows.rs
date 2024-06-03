use musicxml_internal::*;
use musicxml_macros::*;

/// If the [MetronomeArrows] element is present, it indicates that metric modulation arrows are displayed on both sides of the metronome mark.
#[derive(Debug, PartialEq, Eq, ElementDeserialize, ElementSerialize)]
#[rename("metronome-arrows")]
pub struct MetronomeArrows {
  /// Element-specific attributes
  pub attributes: (),
  /// Element-specific content
  pub content: (),
}
