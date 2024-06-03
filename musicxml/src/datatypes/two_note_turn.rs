use musicxml_internal::{DatatypeDeserializer, DatatypeSerializer};
use musicxml_macros::{DatatypeDeserialize, DatatypeSerialize};

/// Describes the ending notes of trills and mordents for playback, relative to the current note.
#[derive(Debug, PartialEq, Eq, DatatypeDeserialize, DatatypeSerialize)]
pub enum TwoNoteTurn {
  /// Half step from the current note.
  Half,
  /// Whole step from the current note.
  Whole,
  /// None of the above.
  None,
}
