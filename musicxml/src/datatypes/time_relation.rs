use musicxml_internal::{DatatypeDeserializer, DatatypeSerializer};
use musicxml_macros::{DatatypeDeserialize, DatatypeSerialize};

/// Indicates the symbol used to represent the interchangeable aspect of dual time signatures.
#[derive(Debug, PartialEq, Eq, DatatypeDeserialize, DatatypeSerialize)]
pub enum TimeRelation {
  /// ![Bracket](https://hedgetechllc.github.io/musicxml/musicxml/datatypes/time-relation-bracket.png)
  Bracket,
  /// ![Equals](https://hedgetechllc.github.io/musicxml/musicxml/datatypes/time-relation-equals.png)
  Equals,
  /// ![Hyphen](https://hedgetechllc.github.io/musicxml/musicxml/datatypes/time-relation-hyphen.png)
  Hyphen,
  /// ![Parentheses](https://hedgetechllc.github.io/musicxml/musicxml/datatypes/time-relation-parentheses.png)
  Parentheses,
  /// ![Slash](https://hedgetechllc.github.io/musicxml/musicxml/datatypes/time-relation-slash.png)
  Slash,
  /// ![Space](https://hedgetechllc.github.io/musicxml/musicxml/datatypes/time-relation-space.png)
  Space,
}
