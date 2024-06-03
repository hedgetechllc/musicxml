use musicxml_internal::{DatatypeDeserializer, DatatypeSerializer};
use musicxml_macros::{DatatypeDeserialize, DatatypeSerialize};

/// Indicates the symbol used to represent the interchangeable aspect of dual time signatures.
#[derive(Debug, PartialEq, Eq, DatatypeDeserialize, DatatypeSerialize)]
pub enum TimeRelation {
  /// ![Bracket](time-relation-bracket.png)
  Bracket,
  /// ![Equals](time-relation-equals.png)
  Equals,
  /// ![Hyphen](time-relation-hyphen.png)
  Hyphen,
  /// ![Parentheses](time-relation-parentheses.png)
  Parentheses,
  /// ![Slash](time-relation-slash.png)
  Slash,
  /// ![Space](time-relation-space.png)
  Space,
}
