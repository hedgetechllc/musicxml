use musicxml_internal::{DatatypeDeserializer, DatatypeSerializer};
use musicxml_macros::{DatatypeDeserialize, DatatypeSerialize};

/// Indicates how the symbol for a group or multi-staff part is indicated in the score.
#[derive(Debug, PartialEq, Eq, DatatypeDeserialize, DatatypeSerialize)]
pub enum GroupSymbolValue {
  /// ![Brace](group-symbol-value-brace.png)
  Brace,
  /// ![Bracket](group-symbol-value-bracket.png)
  Bracket,
  /// ![Line](group-symbol-value-line.png)
  Line,
  /// No symbol is displayed.
  None,
  /// ![Square](group-symbol-value-square.png)
  Square,
}
