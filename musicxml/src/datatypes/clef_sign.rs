use musicxml_internal::{DatatypeDeserializer, DatatypeSerializer};
use musicxml_macros::{DatatypeDeserialize, DatatypeSerialize};

/// Represents the different clef symbols.
#[derive(Debug, PartialEq, Eq, DatatypeDeserialize, DatatypeSerialize)]
pub enum ClefSign {
  /// ![G](clef-G.png)
  #[rename("G")]
  G,
  /// ![F](clef-F.png)
  #[rename("F")]
  F,
  /// ![C](clef-C.png)
  #[rename("C")]
  C,
  /// ![Percussion](clef-percussion.png)
  Percussion,
  /// ![TAB](clef-TAB.png)
  /// 
  /// The TAB sign indicates that the music that follows should be in tablature notation.
  #[rename("TAB")]
  TAB,
  /// The jianpu sign indicates that the music that follows should be in jianpu numbered notation.
  /// Unlike TAB, a jianpu sign does not correspond to the visual clef notation.
  Jianpu,
  /// Deprecated as of MusicXML 4.0. Use the clef element's `print_object` attribute instead.
  /// When the [None][ClefSign::None] sign is used, notes should be displayed as if in treble clef.
  None,
}
