use musicxml_internal::{DatatypeDeserializer, DatatypeSerializer};
use musicxml_macros::{DatatypeDeserialize, DatatypeSerialize};

/// Represents the direction in which an arrow points, using Unicode arrow terminology.
#[derive(Debug, PartialEq, Eq, DatatypeDeserialize, DatatypeSerialize)]
pub enum ArrowDirection {
  /// ![Down](arrow-direction-down.png)
  Down,
  /// ![Left](arrow-direction-left.png)
  Left,
  /// ![LeftRight](arrow-direction-left-right.png)
  #[rename("left right")]
  LeftRight,
  /// ![Northeast](arrow-direction-northeast.png)
  Northeast,
  /// ![NortheastSouthwest](arrow-direction-northeast-southwest.png)
  #[rename("northeast southwest")]
  NortheastSouthwest,
  /// ![Northwest](arrow-direction-northwest.png)
  Northwest,
  /// ![NorthwestSoutheast](arrow-direction-northwest-southeast.png)
  #[rename("northwest southeast")]
  NorthwestSoutheast,
  /// Another direction besides one of those listed.
  Other,
  /// ![Right](arrow-direction-right.png)
  Right,
  /// ![Southeast](arrow-direction-southeast.png)
  Southeast,
  /// ![Southwest](arrow-direction-southwest.png)
  Southwest,
  /// ![Up](arrow-direction-up.png)
  Up,
  /// ![UpDown](arrow-direction-up-down.png)
  #[rename("up down")]
  UpDown,
}
