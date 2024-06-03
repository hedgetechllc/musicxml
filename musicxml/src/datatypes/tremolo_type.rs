use musicxml_internal::{DatatypeDeserializer, DatatypeSerializer};
use musicxml_macros::{DatatypeDeserialize, DatatypeSerialize};

/// Used to distinguish double-note, single-note, and unmeasured tremolos.
#[derive(Debug, PartialEq, Eq, DatatypeDeserialize, DatatypeSerialize)]
pub enum TremoloType {
  /// Start of a double-note tremolo.
  Start,
  /// Stop of a double-note tremolo.
  Stop,
  /// Single-note tremolo.
  Single,
  /// Unmeasured tremolo.
  Unmeasured,
}
