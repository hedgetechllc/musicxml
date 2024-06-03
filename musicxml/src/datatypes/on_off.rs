use musicxml_internal::{DatatypeDeserializer, DatatypeSerializer};
use musicxml_macros::{DatatypeDeserialize, DatatypeSerialize};

/// Ued for notation elements such as string mutes.
#[derive(Debug, PartialEq, Eq, DatatypeDeserialize, DatatypeSerialize)]
pub enum OnOff {
  /// Represents the "on" state.
  On,
  /// Represents the "off" state.
  Off,
}
