use musicxml_internal::{DatatypeDeserializer, DatatypeSerializer};
use musicxml_macros::{DatatypeDeserialize, DatatypeSerialize};

/// Represents the type of beam fanning present on a note, used to represent accelerandos and ritardandos.
#[derive(Debug, PartialEq, Eq, DatatypeDeserialize, DatatypeSerialize)]
pub enum Fan {
  /// ![Accelerando](fan-accel.png)
  Accel,
  /// ![Ritardando](fan-rit.png)
  Rit,
}
