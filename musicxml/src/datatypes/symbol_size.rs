use musicxml_internal::{DatatypeDeserializer, DatatypeSerializer};
use musicxml_macros::{DatatypeDeserialize, DatatypeSerialize};

/// Used to distinguish between full, cue sized, grace cue sized, and oversized symbols.
#[derive(Debug, PartialEq, Eq, DatatypeDeserialize, DatatypeSerialize)]
pub enum SymbolSize {
  /// Use a cue-sized symbol, generally smaller than a full-sized symbol.
  Cue,
  /// Use a full-sized symbol.
  Full,
  /// Use a grace-cue-sized symbol, generally smaller than a cue-sized symbol.
  #[rename("grace-cue")]
  GraceCue,
  /// Use an oversized symbol.
  Large,
}
