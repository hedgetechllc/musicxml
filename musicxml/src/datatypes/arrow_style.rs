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
  /// ![Combined](arrow-style-combined.png)
  Combined,
  /// ![Double](arrow-style-double.png)
  Double,
  /// ![Filled](arrow-style-filled.png)
  Filled,
  /// ![Hollow](arrow-style-hollow.png)
  Hollow,
  /// Another style besides on of those listed.
  Other,
  /// ![Paired](arrow-style-paired.png)
  Paired,
  /// ![Single](arrow-style-single.png)
  Single,
}
