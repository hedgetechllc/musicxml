use musicxml_internal::{DatatypeDeserializer, DatatypeSerializer};
use musicxml_macros::{DatatypeDeserialize, DatatypeSerialize};

/// Describes the shape and presence / absence of an enclosure around text or symbols.
///
/// A [Bracket][EnclosureShape::Bracket] enclosure is similar to a rectangle with the bottom line missing,
/// as is common in jazz notation. An [InvertedBracket][EnclosureShape::InvertedBracket] enclosure is
/// similar to a rectangle with the top line missing.
#[derive(Debug, PartialEq, Eq, DatatypeDeserialize, DatatypeSerialize)]
pub enum EnclosureShape {
  /// ![Rectangle](https://hedgetechllc.github.io/musicxml/musicxml/datatypes/enclosure-shape-rectangle.png)
  Rectangle,
  /// ![Square](https://hedgetechllc.github.io/musicxml/musicxml/datatypes/enclosure-shape-square.png)
  Square,
  /// ![Oval](https://hedgetechllc.github.io/musicxml/musicxml/datatypes/enclosure-shape-oval.png)
  Oval,
  /// ![Circle](https://hedgetechllc.github.io/musicxml/musicxml/datatypes/enclosure-shape-circle.png)
  Circle,
  /// ![Bracket](https://hedgetechllc.github.io/musicxml/musicxml/datatypes/enclosure-shape-bracket.png)
  Bracket,
  /// ![Inverted Bracket](https://hedgetechllc.github.io/musicxml/musicxml/datatypes/enclosure-shape-inverted-bracket.png)
  #[rename("inverted-bracket")]
  InvertedBracket,
  /// ![Triangle](https://hedgetechllc.github.io/musicxml/musicxml/datatypes/enclosure-shape-triangle.png)
  Triangle,
  /// ![Diamond](https://hedgetechllc.github.io/musicxml/musicxml/datatypes/enclosure-shape-diamond.png)
  Diamond,
  /// ![Trill](https://hedgetechllc.github.io/musicxml/musicxml/datatypes/enclosure-shape-pentagon.png)
  Pentagon,
  /// ![Hexagon](https://hedgetechllc.github.io/musicxml/musicxml/datatypes/enclosure-shape-hexagon.png)
  Hexagon,
  /// ![Heptagon](https://hedgetechllc.github.io/musicxml/musicxml/datatypes/enclosure-shape-heptagon.png)
  Heptagon,
  /// ![Octagon](https://hedgetechllc.github.io/musicxml/musicxml/datatypes/enclosure-shape-octagon.png)
  Octagon,
  /// ![Nonagon](https://hedgetechllc.github.io/musicxml/musicxml/datatypes/enclosure-shape-nonagon.png)
  Nonagon,
  /// ![Decagon](https://hedgetechllc.github.io/musicxml/musicxml/datatypes/enclosure-shape-decagon.png)
  Decagon,
  /// No enclosure is displayed.
  None,
}
