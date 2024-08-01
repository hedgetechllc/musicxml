use alloc::string::String;
use musicxml_internal::{DatatypeDeserializer, DatatypeSerializer};
use musicxml_macros::{DatatypeDeserialize, DatatypeSerialize};

/// Represents the direction in which the tip of a stick or beater points, using Unicode arrow terminology.
#[derive(Debug, PartialEq, Eq, DatatypeDeserialize, DatatypeSerialize)]
pub enum TipDirection {
  /// ![Down](https://hedgetechllc.github.io/musicxml/musicxml/datatypes/tip-direction-down.png)
  Down,
  /// ![Left](https://hedgetechllc.github.io/musicxml/musicxml/datatypes/tip-direction-left.png)
  Left,
  /// ![Northeast](https://hedgetechllc.github.io/musicxml/musicxml/datatypes/tip-direction-northeast.png)
  Northeast,
  /// ![Northwest](https://hedgetechllc.github.io/musicxml/musicxml/datatypes/tip-direction-northwest.png)
  Northwest,
  /// ![Right](https://hedgetechllc.github.io/musicxml/musicxml/datatypes/tip-direction-right.png)
  Right,
  /// ![Southeast](https://hedgetechllc.github.io/musicxml/musicxml/datatypes/tip-direction-southeast.png)
  Southeast,
  /// ![Southwest](https://hedgetechllc.github.io/musicxml/musicxml/datatypes/tip-direction-southwest.png)
  Southwest,
  /// ![Up](https://hedgetechllc.github.io/musicxml/musicxml/datatypes/tip-direction-up.png)
  Up,
}
