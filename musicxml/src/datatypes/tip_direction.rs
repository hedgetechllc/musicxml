use musicxml_internal::{DatatypeDeserializer, DatatypeSerializer};
use musicxml_macros::{DatatypeDeserialize, DatatypeSerialize};

/// Represents the direction in which the tip of a stick or beater points, using Unicode arrow terminology.
#[derive(Debug, PartialEq, Eq, DatatypeDeserialize, DatatypeSerialize)]
pub enum TipDirection {
  /// ![Down](tip-direction-down.png)
  Down,
  /// ![Left](tip-direction-left.png)
  Left,
  /// ![Northeast](tip-direction-northeast.png)
  Northeast,
  /// ![Northwest](tip-direction-northwest.png)
  Northwest,
  /// ![Right](tip-direction-right.png)
  Right,
  /// ![Southeast](tip-direction-southeast.png)
  Southeast,
  /// ![Southwest](tip-direction-southwest.png)
  Southwest,
  /// ![Up](tip-direction-up.png)
  Up,
}
