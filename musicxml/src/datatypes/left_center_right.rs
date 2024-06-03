use musicxml_internal::{DatatypeDeserializer, DatatypeSerializer};
use musicxml_macros::{DatatypeDeserialize, DatatypeSerialize};

/// Used to define horizontal alignment and text justification.
#[derive(Debug, PartialEq, Eq, DatatypeDeserialize, DatatypeSerialize)]
pub enum LeftCenterRight {
  /// Left horizontal alignment or justification.
  Left,
  /// Center horizontal alignment or justification.
  Center,
  /// Right horizontal alignment or justification.
  Right,
}
