use musicxml_internal::{DatatypeDeserializer, DatatypeSerializer};
use musicxml_macros::{DatatypeDeserialize, DatatypeSerialize};

/// Distinguishes elements that are associated with a system rather than the particular part where the element appears.
#[derive(Debug, PartialEq, Eq, DatatypeDeserialize, DatatypeSerialize)]
pub enum SystemRelation {
  /// The element should appear only on the top part of the current system.
  #[rename("only-top")]
  OnlyTop,
  /// The element should appear on both the current part and the top part of the current system.
  /// 
  /// If this value appears in a score, when parts are created the element should only appear once in this part, not twice.
  #[rename("also-top")]
  AlsoTop,
  /// The element is associated only with the current part, not with the system.
  None,
}
