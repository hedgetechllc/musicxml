use musicxml_internal::{DatatypeDeserializer, DatatypeSerializer};
use musicxml_macros::{DatatypeDeserialize, DatatypeSerialize};

/// Specifies if there is a jog up or down (or both), an arrow, or nothing at the start or end of a bracket.
#[derive(Debug, PartialEq, Eq, DatatypeDeserialize, DatatypeSerialize)]
pub enum LineEnd {
  /// ![Up](line-end-up.png)
  Up,
  /// ![Down](line-end-down.png)
  Down,
  /// ![Up and Down](line-end-both.png)
  Both,
  /// ![Arrow](line-end-arrow.png)
  Arrow,
  /// ![None](line-end-none.png)
  None,
}
