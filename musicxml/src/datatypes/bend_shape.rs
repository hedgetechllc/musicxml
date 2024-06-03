use musicxml_internal::{DatatypeDeserializer, DatatypeSerializer};
use musicxml_macros::{DatatypeDeserialize, DatatypeSerialize};

/// Distinguishes between the [Angled][BendShape::Angled] bend symbols commonly used in standard notation
/// and the [Curved][BendShape::Curved] bend symbols commonly used in both tablature and standard notation.
#[derive(Debug, PartialEq, Eq, DatatypeDeserialize, DatatypeSerialize)]
pub enum BendShape {
  /// ![Angled](bend-shape-angled.png)
  Angled,
  /// ![Curved](bend-shape-curved.png)
  Curved,
}
