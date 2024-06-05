use musicxml_internal::{DatatypeDeserializer, DatatypeSerializer};
use musicxml_macros::{DatatypeDeserialize, DatatypeSerialize};

/// Indicates whether the repeat has winged extensions that appear above and below the barline.
///
/// The [Straight][Winged::Straight] and [Curved][Winged::Curved] values represent single wings, while the
/// [DoubleStraight][Winged::DoubleStraight] and [DoubleCurved][Winged::DoubleCurved] values represent
/// double wings. The [None][Winged::None] value indicates no wings and is the default.
#[derive(Debug, PartialEq, Eq, DatatypeDeserialize, DatatypeSerialize)]
pub enum Winged {
  /// ![None](https://hedgetechllc.github.io/musicxml/musicxml/datatypes/winged-none.png)
  None,
  /// ![Straight](https://hedgetechllc.github.io/musicxml/musicxml/datatypes/winged-straight.png)
  Straight,
  /// ![Curved](https://hedgetechllc.github.io/musicxml/musicxml/datatypes/winged-curved.png)
  Curved,
  /// ![DoubleStraight](https://hedgetechllc.github.io/musicxml/musicxml/datatypes/winged-double-straight.png)
  #[rename("double-straight")]
  DoubleStraight,
  /// ![DoubleCurved](https://hedgetechllc.github.io/musicxml/musicxml/datatypes/winged-double-curved.png)
  #[rename("double-curved")]
  DoubleCurved,
}
