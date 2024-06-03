use musicxml_internal::{DatatypeDeserializer, DatatypeSerializer};
use musicxml_macros::{DatatypeDeserialize, DatatypeSerialize};

/// Used for an attribute of musical elements that can be used for either multi-note or single-note musical elements, as for groupings.
/// 
/// When multiple elements with the same tag are used within the same note, their order within the MusicXML document should match the musical score order.
#[derive(Debug, PartialEq, Eq, DatatypeDeserialize, DatatypeSerialize)]
pub enum StartStopSingle {
  /// Starting point of a multi-note element.
  Start,
  /// Stopping point of a multi-note element.
  Stop,
  /// Single-note element.
  Single,
}
