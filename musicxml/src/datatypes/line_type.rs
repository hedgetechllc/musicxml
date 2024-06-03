use musicxml_internal::{DatatypeDeserializer, DatatypeSerializer};
use musicxml_macros::{DatatypeDeserialize, DatatypeSerialize};

/// Distinguishes between solid, dashed, dotted, and wavy lines.
#[derive(Debug, PartialEq, Eq, DatatypeDeserialize, DatatypeSerialize)]
pub enum LineType {
  /// ![Dashed](line-type-dashed.png)
  Dashed,
  /// ![Dotted](line-type-dotted.png)
  Dotted,
  /// ![Solid](line-type-solid.png)
  Solid,
  /// ![Wavy](line-type-wavy.png)
  Wavy,
}
