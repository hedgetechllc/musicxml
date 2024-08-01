use alloc::string::String;
use musicxml_internal::{DatatypeDeserializer, DatatypeSerializer};
use musicxml_macros::{DatatypeDeserialize, DatatypeSerialize};

/// Represents a simplified version of the [CSS font-style property](https://www.w3.org/TR/2018/REC-css-fonts-3-20180920/#font-prop-desc).
#[derive(Debug, PartialEq, Eq, DatatypeDeserialize, DatatypeSerialize)]
pub enum FontStyle {
  /// Normal text.
  Normal,
  /// Italicized text.
  Italic,
}
