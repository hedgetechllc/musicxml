use musicxml_internal::{DatatypeDeserializer, DatatypeSerializer};
use musicxml_macros::{DatatypeDeserialize, DatatypeSerialize};

/// Distinguishes between the [Angled][BendShape::Angled] bend symbols commonly used in standard notation
/// and the [Curved][BendShape::Curved] bend symbols commonly used in both tablature and standard notation.
#[derive(Debug, PartialEq, Eq, DatatypeDeserialize, DatatypeSerialize)]
pub enum BendShape {
  /// ![Angled](https://hedgetechllc.github.io/musicxml/musicxml/datatypes/bend-shape-angled.png)
  Angled,
  /// ![Curved](https://hedgetechllc.github.io/musicxml/musicxml/datatypes/bend-shape-curved.png)
  Curved,
}
