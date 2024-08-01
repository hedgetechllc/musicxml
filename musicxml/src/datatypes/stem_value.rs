use alloc::string::String;
use musicxml_internal::{DatatypeDeserializer, DatatypeSerializer};
use musicxml_macros::{DatatypeDeserialize, DatatypeSerialize};

/// Represents the notated stem direction.
#[derive(Debug, PartialEq, Eq, DatatypeDeserialize, DatatypeSerialize)]
pub enum StemValue {
  /// ![Down](https://hedgetechllc.github.io/musicxml/musicxml/datatypes/stem-value-down.png)
  Down,
  /// ![Up](https://hedgetechllc.github.io/musicxml/musicxml/datatypes/stem-value-up.png)
  Up,
  /// ![Double](https://hedgetechllc.github.io/musicxml/musicxml/datatypes/stem-value-double.png)
  Double,
  /// No stem appears.
  None,
}
