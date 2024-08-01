use alloc::string::String;
use musicxml_internal::{DatatypeDeserializer, DatatypeSerializer};
use musicxml_macros::{DatatypeDeserialize, DatatypeSerialize};

/// Used to indicate whether the tips of curved lines such as slurs and ties
/// are overhand (tips down) or underhand (tips up).
#[derive(Debug, PartialEq, Eq, DatatypeDeserialize, DatatypeSerialize)]
pub enum OverUnder {
  /// Tips of curved lines are overhand (tips down).
  Over,
  /// Tips of curved lines are underhand (tips up).
  Under,
}
