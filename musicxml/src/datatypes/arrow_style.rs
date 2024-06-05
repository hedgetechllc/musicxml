use musicxml_internal::{DatatypeDeserializer, DatatypeSerializer};
use musicxml_macros::{DatatypeDeserialize, DatatypeSerialize};

/// Represents the style of an arrow, using Unicode arrow terminology.
///
/// [Filled][ArrowStyle::Filled] and [Hollow][ArrowStyle::Hollow] arrows indicate polygonal single arrows.
/// [Paired][ArrowStyle::Paired] arrows are duplicate [Single][ArrowStyle::Single] arrows in the same direction.
/// [Combined][ArrowStyle::Combined] arrows apply to [Double][ArrowStyle::Double] direction arrows like left right,
/// indicating that an arrow in one direction should be combined with an arrow in the other direction.
#[derive(Debug, PartialEq, Eq, DatatypeDeserialize, DatatypeSerialize)]
pub enum ArrowStyle {
  /// ![Combined](https://hedgetechllc.github.io/musicxml/musicxml/datatypes/arrow-style-combined.png)
  Combined,
  /// ![Double](https://hedgetechllc.github.io/musicxml/musicxml/datatypes/arrow-style-double.png)
  Double,
  /// ![Filled](https://hedgetechllc.github.io/musicxml/musicxml/datatypes/arrow-style-filled.png)
  Filled,
  /// ![Hollow](https://hedgetechllc.github.io/musicxml/musicxml/datatypes/arrow-style-hollow.png)
  Hollow,
  /// Another style besides on of those listed.
  Other,
  /// ![Paired](https://hedgetechllc.github.io/musicxml/musicxml/datatypes/arrow-style-paired.png)
  Paired,
  /// ![Single](https://hedgetechllc.github.io/musicxml/musicxml/datatypes/arrow-style-single.png)
  Single,
}
