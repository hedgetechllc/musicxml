use alloc::string::String;
use musicxml_internal::{DatatypeDeserializer, DatatypeSerializer};
use musicxml_macros::{DatatypeDeserialize, DatatypeSerialize};

/// Describes the alternating note of trills and mordents for playback, relative to the current note.
#[derive(Debug, PartialEq, Eq, DatatypeDeserialize, DatatypeSerialize)]
pub enum TrillStep {
  /// Half step from the current note.
  Half,
  /// In unison with the current note.
  Unison,
  /// Whole step from the current note.
  Whole,
}
