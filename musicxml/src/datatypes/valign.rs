use alloc::string::String;
use musicxml_internal::{DatatypeDeserializer, DatatypeSerializer};
use musicxml_macros::{DatatypeDeserialize, DatatypeSerialize};

/// Used to indicate vertical alignment to the top, middle, bottom, or baseline of the text.
///
/// Defaults are implementation-dependent.
#[derive(Debug, PartialEq, Eq, DatatypeDeserialize, DatatypeSerialize)]
pub enum Valign {
  /// Aligned to the top of the text.
  Top,
  /// Aligned to the middle of the text.
  Middle,
  /// Aligned to the bottom of the text.
  Bottom,
  /// Aligned to the baseline of the text.
  ///
  /// If the text is on multiple lines, algned to the baseline of the lowest line of text.
  Baseline,
}
