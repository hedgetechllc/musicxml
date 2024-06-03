use super::{BeatUnit, BeatUnitDot};
use musicxml_internal::*;
use musicxml_macros::*;

/// Contents of the [BeatUnitTied] element.
#[derive(Debug, PartialEq, Eq, ContentDeserialize, ContentSerialize)]
pub struct BeatUnitTiedContents {
  /// The [BeatUnit] element specifies the beat unit for a metronome mark.
  pub beat_unit: BeatUnit,
  /// The [BeatUnitDot] element specifies the presence of a dot in the beat unit.
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
