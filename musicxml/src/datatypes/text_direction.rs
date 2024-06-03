use musicxml_internal::{DatatypeDeserializer, DatatypeSerializer};
use musicxml_macros::{DatatypeDeserialize, DatatypeSerialize};

/// Used to adjust and override the Unicode bidirectional text algorithm,similar to the Directionality
/// data category in the [W3C Internationalization Tag Set recommendation](https://www.w3.org/TR/2007/REC-its-20070403/#directionality).
#[derive(Debug, PartialEq, Eq, DatatypeDeserialize, DatatypeSerialize)]
pub enum TextDirection {
  /// Left-to-right embed.
  Ltr,
  /// Right-to-left embed.
  Rtl,
  /// Left-to-right bidi-override.
  Lro,
  /// Right-to-left bidi-override.
  Rlo,
}
