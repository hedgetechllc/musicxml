use musicxml_internal::{DatatypeDeserializer, DatatypeSerializer};
use musicxml_macros::{DatatypeDeserialize, DatatypeSerialize};

/// Indicates whether the repeat has winged extensions that appear above and below the barline.
///
/// The [Straight][Winged::Straight] and [Curved][Winged::Curved] values represent single wings, while the
/// [DoubleStraight][Winged::DoubleStraight] and [DoubleCurved][Winged::DoubleCurved] values represent
/// double wings. The [None][Winged::None] value indicates no wings and is the default.
#[derive(Debug, PartialEq, Eq, DatatypeDeserialize, DatatypeSerialize)]
pub enum Winged {
  /// ![None](winged-none.png)
  None,
  /// ![Straight](winged-straight.png)
  Straight,
  /// ![Curved](winged-curved.png)
  Curved,
  /// ![DoubleStraight](winged-double-straight.png)
  #[rename("double-straight")]
  DoubleStraight,
  /// ![DoubleCurved](winged-double-curved.png)
  #[rename("double-curved")]
  DoubleCurved,
}
