use musicxml_internal::{DatatypeDeserializer, DatatypeSerializer};
use musicxml_macros::{DatatypeDeserialize, DatatypeSerialize};

/// Distinguishes between straight and curved lines.
#[derive(Debug, PartialEq, Eq, DatatypeDeserialize, DatatypeSerialize)]
pub enum LineShape {
  /// <span class="smufl">&#xE5DA;</span>
  Straight,
  /// <span class="smufl">&#xE5D8;</span>
  Curved,
}
