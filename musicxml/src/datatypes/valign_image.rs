use musicxml_internal::{DatatypeDeserializer, DatatypeSerializer};
use musicxml_macros::{DatatypeDeserialize, DatatypeSerialize};

/// Used to indicate vertical alignment for images and graphics, so it does not include a baseline value.
///
/// Defaults are implementation-dependent.
#[derive(Debug, PartialEq, Eq, DatatypeDeserialize, DatatypeSerialize)]
pub enum ValignImage {
  /// Aligned to the top of the image.
  Top,
  /// Aligned to the middle of the image.
  Middle,
  /// Aligned to the bottom of the image.
  Bottom,
}
