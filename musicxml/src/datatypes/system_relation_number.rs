use musicxml_internal::{DatatypeDeserializer, DatatypeSerializer};
use musicxml_macros::{DatatypeDeserialize, DatatypeSerialize};

/// Distinguishes measure numbers that are associated with a system rather than the particular part where the element appears.
#[derive(Debug, PartialEq, Eq, DatatypeDeserialize, DatatypeSerialize)]
pub enum SystemRelationNumber {
  /// The number should appear only on the top part of the current system.
  #[rename("only-top")]
  OnlyTop,
  /// The number should appear only on the bottom part of the current system.
  #[rename("only-bottom")]
  OnlyBottom,
  /// The number should appear on both the current part and the top part of the current system.
  ///
  /// If these values appear in a score, when parts are created the number should only appear once in this part, not twice.
  #[rename("also-top")]
  AlsoTop,
  /// The number should appear on both the current part and the bottom part of the current system.
  ///
  /// If these values appear in a score, when parts are created the number should only appear once in this part, not twice.
  #[rename("also-bottom")]
  AlsoBottom,
  /// The number is associated only with the current part, not with the system.
  None,
}
