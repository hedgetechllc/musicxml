use musicxml_internal::{DatatypeDeserializer, DatatypeSerializer};
use musicxml_macros::{DatatypeDeserialize, DatatypeSerialize};

/// Distinguishes between solid, dashed, dotted, and wavy lines.
#[derive(Debug, PartialEq, Eq, DatatypeDeserialize, DatatypeSerialize)]
pub enum LineType {
  /// ![Dashed](https://hedgetechllc.github.io/musicxml/musicxml/datatypes/line-type-dashed.png)
  Dashed,
  /// ![Dotted](https://hedgetechllc.github.io/musicxml/musicxml/datatypes/line-type-dotted.png)
  Dotted,
  /// ![Solid](https://hedgetechllc.github.io/musicxml/musicxml/datatypes/line-type-solid.png)
  Solid,
  /// ![Wavy](https://hedgetechllc.github.io/musicxml/musicxml/datatypes/line-type-wavy.png)
  Wavy,
}
