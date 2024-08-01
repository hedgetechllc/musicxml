use alloc::string::String;
use musicxml_internal::{DatatypeDeserializer, DatatypeSerializer};
use musicxml_macros::{DatatypeDeserialize, DatatypeSerialize};

/// Represents the different clef symbols.
#[derive(Debug, PartialEq, Eq, DatatypeDeserialize, DatatypeSerialize)]
pub enum ClefSign {
  /// ![G](https://hedgetechllc.github.io/musicxml/musicxml/datatypes/clef-G.png)
  #[rename("G")]
  G,
  /// ![F](https://hedgetechllc.github.io/musicxml/musicxml/datatypes/clef-F.png)
  #[rename("F")]
  F,
  /// ![C](https://hedgetechllc.github.io/musicxml/musicxml/datatypes/clef-C.png)
  #[rename("C")]
  C,
  /// ![Percussion](https://hedgetechllc.github.io/musicxml/musicxml/datatypes/clef-percussion.png)
  Percussion,
  /// ![TAB](https://hedgetechllc.github.io/musicxml/musicxml/datatypes/clef-TAB.png)
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
