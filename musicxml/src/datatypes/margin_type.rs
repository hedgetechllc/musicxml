use alloc::string::String;
use musicxml_internal::{DatatypeDeserializer, DatatypeSerializer};
use musicxml_macros::{DatatypeDeserialize, DatatypeSerialize};

/// Specifies whether margins apply to even page, odd pages, or both.
#[derive(Debug, PartialEq, Eq, DatatypeDeserialize, DatatypeSerialize)]
pub enum MarginType {
  /// Margins apply to both even and odd pages.
  Both,
  /// Margins apply to even pages only.
  Even,
  /// Margins apply to odd pages only.
  Odd,
}
