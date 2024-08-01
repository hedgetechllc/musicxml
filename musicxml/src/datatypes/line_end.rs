use alloc::string::String;
use musicxml_internal::{DatatypeDeserializer, DatatypeSerializer};
use musicxml_macros::{DatatypeDeserialize, DatatypeSerialize};

/// Specifies if there is a jog up or down (or both), an arrow, or nothing at the start or end of a bracket.
#[derive(Debug, PartialEq, Eq, DatatypeDeserialize, DatatypeSerialize)]
pub enum LineEnd {
  /// ![Up](https://hedgetechllc.github.io/musicxml/musicxml/datatypes/line-end-up.png)
  Up,
  /// ![Down](https://hedgetechllc.github.io/musicxml/musicxml/datatypes/line-end-down.png)
  Down,
  /// ![Up and Down](https://hedgetechllc.github.io/musicxml/musicxml/datatypes/line-end-both.png)
  Both,
  /// ![Arrow](https://hedgetechllc.github.io/musicxml/musicxml/datatypes/line-end-arrow.png)
  Arrow,
  /// ![None](https://hedgetechllc.github.io/musicxml/musicxml/datatypes/line-end-none.png)
  None,
}
