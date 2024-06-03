use musicxml_internal::{DatatypeDeserializer, DatatypeSerializer};
use musicxml_macros::{DatatypeDeserialize, DatatypeSerialize};

/// Indicates whether the current degree element is an addition, alteration, or subtraction
/// to the kind of the current chord in the harmony element.
#[derive(Debug, PartialEq, Eq, DatatypeDeserialize, DatatypeSerialize)]
pub enum DegreeTypeValue {
  /// The degree is an addition to the kind of the current chord.
  Add,
  /// The degree is an alteration to the kind of the current chord.
  Alter,
  /// The degree is a subtraction from the kind of the current chord.
  Subtract,
}
