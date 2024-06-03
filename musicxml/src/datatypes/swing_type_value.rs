use musicxml_internal::{DatatypeDeserializer, DatatypeSerializer};
use musicxml_macros::{DatatypeDeserialize, DatatypeSerialize};

/// Specifies the note type, either eighth or 16th, to which the ratio defined in the [Swing][crate::elements::Swing] element is applied.
#[derive(Debug, PartialEq, Eq, DatatypeDeserialize, DatatypeSerialize)]
pub enum SwingTypeValue {
  /// The [Swing][crate::elements::Swing] ratio applies to 16th notes.
  #[rename("16th")]
  Sixteenth,
  /// The[Swing][crate::elements::Swing] ratio applies to eighth notes.
  Eighth,
}
