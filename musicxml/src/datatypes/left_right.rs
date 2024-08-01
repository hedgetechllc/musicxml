use alloc::string::String;
use musicxml_internal::{DatatypeDeserializer, DatatypeSerializer};
use musicxml_macros::{DatatypeDeserialize, DatatypeSerialize};

/// Used to indicate whether one element appears to the left or the right of another element.
#[derive(Debug, PartialEq, Eq, DatatypeDeserialize, DatatypeSerialize)]
pub enum LeftRight {
  /// This element appears to the left of the reference element.
  Left,
  /// This element appears to the right of the reference element.
  Right,
}
