use musicxml_internal::{DatatypeDeserializer, DatatypeSerializer};
use musicxml_macros::{DatatypeDeserialize, DatatypeSerialize};

/// Describes the starting note of trills and mordents for playback, relative to the current note.
#[derive(Debug, PartialEq, Eq, DatatypeDeserialize, DatatypeSerialize)]
pub enum StartNote {
  /// The trill or mordent starts on the note below the current note.
  Below,
  /// The trill or mordent starts on the current note.
  Main,
  /// The trill or mordent starts on the note above the current note.
  Upper,
}
