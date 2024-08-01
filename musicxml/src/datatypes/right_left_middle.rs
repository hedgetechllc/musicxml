use alloc::string::String;
use musicxml_internal::{DatatypeDeserializer, DatatypeSerializer};
use musicxml_macros::{DatatypeDeserialize, DatatypeSerialize};

/// Used to specify barline location.
#[derive(Debug, PartialEq, Eq, DatatypeDeserialize, DatatypeSerialize)]
pub enum RightLeftMiddle {
  /// Right barline.
  Right,
  /// Left barline.
  Left,
  /// Mid-measure barline.
  Middle,
}
