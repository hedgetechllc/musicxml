use musicxml_internal::{DatatypeDeserializer, DatatypeSerializer};
use musicxml_macros::{DatatypeDeserialize, DatatypeSerialize};

/// Represents the direction in which a circular arrow points, using Unicode arrow terminology.
#[derive(Debug, PartialEq, Eq, DatatypeDeserialize, DatatypeSerialize)]
pub enum CircularArrow {
  /// ![Anticlockwise](circular-arrow-anticlockwise.png)
  Anticlockwise,
  /// ![Clockwise](circular-arrow-clockwise.png)
  Clockwise,
}
