use alloc::string::String;
use musicxml_internal::{DatatypeDeserializer, DatatypeSerializer};
use musicxml_macros::{DatatypeDeserialize, DatatypeSerialize};

/// Represents the type of beam fanning present on a note, used to represent accelerandos and ritardandos.
#[derive(Debug, PartialEq, Eq, DatatypeDeserialize, DatatypeSerialize)]
pub enum Fan {
  /// ![Accelerando](https://hedgetechllc.github.io/musicxml/musicxml/datatypes/fan-accel.png)
  Accel,
  /// ![Ritardando](https://hedgetechllc.github.io/musicxml/musicxml/datatypes/fan-rit.png)
  Rit,
}
