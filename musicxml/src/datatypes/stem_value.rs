use musicxml_internal::{DatatypeDeserializer, DatatypeSerializer};
use musicxml_macros::{DatatypeDeserialize, DatatypeSerialize};

/// Represents the notated stem direction.
#[derive(Debug, PartialEq, Eq, DatatypeDeserialize, DatatypeSerialize)]
pub enum StemValue {
  /// ![Down](stem-value-down.png)
  Down,
  /// ![Up](stem-value-up.png)
  Up,
  /// ![Double](stem-value-double.png)
  Double,
  /// No stem appears.
  None,
}
