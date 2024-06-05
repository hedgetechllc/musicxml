use musicxml_internal::{DatatypeDeserializer, DatatypeSerializer};
use musicxml_macros::{DatatypeDeserialize, DatatypeSerialize};

/// Indicates how the symbol for a group or multi-staff part is indicated in the score.
#[derive(Debug, PartialEq, Eq, DatatypeDeserialize, DatatypeSerialize)]
pub enum GroupSymbolValue {
  /// ![Brace](https://hedgetechllc.github.io/musicxml/musicxml/datatypes/group-symbol-value-brace.png)
  Brace,
  /// ![Bracket](https://hedgetechllc.github.io/musicxml/musicxml/datatypes/group-symbol-value-bracket.png)
  Bracket,
  /// ![Line](https://hedgetechllc.github.io/musicxml/musicxml/datatypes/group-symbol-value-line.png)
  Line,
  /// No symbol is displayed.
  None,
  /// ![Square](https://hedgetechllc.github.io/musicxml/musicxml/datatypes/group-symbol-value-square.png)
  Square,
}
