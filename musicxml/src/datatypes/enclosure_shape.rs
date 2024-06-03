use musicxml_internal::{DatatypeDeserializer, DatatypeSerializer};
use musicxml_macros::{DatatypeDeserialize, DatatypeSerialize};

/// Describes the shape and presence / absence of an enclosure around text or symbols.
///
/// A [Bracket][EnclosureShape::Bracket] enclosure is similar to a rectangle with the bottom line missing,
/// as is common in jazz notation. An [InvertedBracket][EnclosureShape::InvertedBracket] enclosure is
/// similar to a rectangle with the top line missing.
#[derive(Debug, PartialEq, Eq, DatatypeDeserialize, DatatypeSerialize)]
pub enum EnclosureShape {
  /// ![Rectangle](enclosure-shape-rectangle.png)
  Rectangle,
  /// ![Square](enclosure-shape-square.png)
  Square,
  /// ![Oval](enclosure-shape-oval.png)
  Oval,
  /// ![Circle](enclosure-shape-circle.png)
  Circle,
  /// ![Bracket](enclosure-shape-bracket.png)
  Bracket,
  /// ![Inverted Bracket](enclosure-shape-inverted-bracket.png)
  #[rename("inverted-bracket")]
  InvertedBracket,
  /// ![Triangle](enclosure-shape-triangle.png)
  Triangle,
  /// ![Diamond](enclosure-shape-diamond.png)
  Diamond,
  /// ![Trill](enclosure-shape-pentagon.png)
  Pentagon,
  /// ![Hexagon](enclosure-shape-hexagon.png)
  Hexagon,
  /// ![Heptagon](enclosure-shape-heptagon.png)
  Heptagon,
  /// ![Octagon](enclosure-shape-octagon.png)
  Octagon,
  /// ![Nonagon](enclosure-shape-nonagon.png)
  Nonagon,
  /// ![Decagon](enclosure-shape-decagon.png)
  Decagon,
  /// No enclosure is displayed.
  None,
}
