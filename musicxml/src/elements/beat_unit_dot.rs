use musicxml_internal::*;
use musicxml_macros::*;

/// The [BeatUnitDot] element is used to specify any augmentation dots for a metronome mark note.
#[derive(Debug, PartialEq, Eq, ElementDeserialize, ElementSerialize)]
#[rename("beat-unit-dot")]
pub struct BeatUnitDot {
  /// Element-specific attributes
  pub attributes: (),
  /// Element-specific content
  pub content: (),
}
