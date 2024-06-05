use musicxml_internal::{DatatypeDeserializer, DatatypeSerializer};
use musicxml_macros::{DatatypeDeserialize, DatatypeSerialize};

/// Represents the direction in which an arrow points, using Unicode arrow terminology.
#[derive(Debug, PartialEq, Eq, DatatypeDeserialize, DatatypeSerialize)]
pub enum ArrowDirection {
  /// ![Down](https://hedgetechllc.github.io/musicxml/musicxml/datatypes/arrow-direction-down.png)
  Down,
  /// ![Left](https://hedgetechllc.github.io/musicxml/musicxml/datatypes/arrow-direction-left.png)
  Left,
  /// ![LeftRight](https://hedgetechllc.github.io/musicxml/musicxml/datatypes/arrow-direction-left-right.png)
  #[rename("left right")]
  LeftRight,
  /// ![Northeast](https://hedgetechllc.github.io/musicxml/musicxml/datatypes/arrow-direction-northeast.png)
  Northeast,
  /// ![NortheastSouthwest](https://hedgetechllc.github.io/musicxml/musicxml/datatypes/arrow-direction-northeast-southwest.png)
  #[rename("northeast southwest")]
  NortheastSouthwest,
  /// ![Northwest](https://hedgetechllc.github.io/musicxml/musicxml/datatypes/arrow-direction-northwest.png)
  Northwest,
  /// ![NorthwestSoutheast](https://hedgetechllc.github.io/musicxml/musicxml/datatypes/arrow-direction-northwest-southeast.png)
  #[rename("northwest southeast")]
  NorthwestSoutheast,
  /// Another direction besides one of those listed.
  Other,
  /// ![Right](https://hedgetechllc.github.io/musicxml/musicxml/datatypes/arrow-direction-right.png)
  Right,
  /// ![Southeast](https://hedgetechllc.github.io/musicxml/musicxml/datatypes/arrow-direction-southeast.png)
  Southeast,
  /// ![Southwest](https://hedgetechllc.github.io/musicxml/musicxml/datatypes/arrow-direction-southwest.png)
  Southwest,
  /// ![Up](https://hedgetechllc.github.io/musicxml/musicxml/datatypes/arrow-direction-up.png)
  Up,
  /// ![UpDown](https://hedgetechllc.github.io/musicxml/musicxml/datatypes/arrow-direction-up-down.png)
  #[rename("up down")]
  UpDown,
}
