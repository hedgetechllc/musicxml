use super::{BeatUnit, BeatUnitDot};
use musicxml_internal::*;
use musicxml_macros::*;

#[derive(Debug, PartialEq, Eq, ContentDeserialize, ContentSerialize)]
pub struct BeatUnitTiedContents {
  pub beat_unit: BeatUnit,
  pub beat_unit_dot: Vec<BeatUnitDot>,
}

/// The [BeatUnitTied] element indicates a [BeatUnit] within a metronome mark that is tied to the preceding [BeatUnit].
/// 
/// This allows two or more tied notes to be associated with a [PerMinute][super::PerMinute] value in a metronome mark, whereas the [MetronomeTied][super::MetronomeTied] element
/// is restricted to metric relationship marks.
#[derive(Debug, PartialEq, Eq, ElementDeserialize, ElementSerialize)]
#[rename("beat-unit-tied")]
pub struct BeatUnitTied {
  /// Element-specific attributes
  pub attributes: (),
  #[flatten]
  /// Element-specific content
  pub content: BeatUnitTiedContents,
}
