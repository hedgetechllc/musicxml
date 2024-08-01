use alloc::string::String;
use musicxml_internal::{DatatypeDeserializer, DatatypeSerializer};
use musicxml_macros::{DatatypeDeserialize, DatatypeSerialize};

/// Used for the MusicXML [Type][crate::elements::Type] element and represents the graphic note type,
/// from 1024th (shortest) to maxima (longest).
#[derive(Debug, PartialEq, Eq, DatatypeDeserialize, DatatypeSerialize)]
pub enum NoteTypeValue {
  /// ![Maxima](https://hedgetechllc.github.io/musicxml/musicxml/datatypes/note-type-maxima.png)
  Maxima,
  /// ![Long](https://hedgetechllc.github.io/musicxml/musicxml/datatypes/note-type-long.png)
  Long,
  /// ![Breve](https://hedgetechllc.github.io/musicxml/musicxml/datatypes/note-type-breve.png)
  Breve,
  /// ![Whole](https://hedgetechllc.github.io/musicxml/musicxml/datatypes/note-type-whole.png)
  Whole,
  /// ![Half](https://hedgetechllc.github.io/musicxml/musicxml/datatypes/note-type-half.png)
  Half,
  /// ![Quarter](https://hedgetechllc.github.io/musicxml/musicxml/datatypes/note-type-quarter.png)
  Quarter,
  /// ![Eighth](https://hedgetechllc.github.io/musicxml/musicxml/datatypes/note-type-eighth.png)
  Eighth,
  /// ![16th](https://hedgetechllc.github.io/musicxml/musicxml/datatypes/note-type-16th.png)
  #[rename("16th")]
  Sixteenth,
  /// ![32nd](https://hedgetechllc.github.io/musicxml/musicxml/datatypes/note-type-32nd.png)
  #[rename("32th")]
  ThirtySecond,
  /// ![64th](https://hedgetechllc.github.io/musicxml/musicxml/datatypes/note-type-64th.png)
  #[rename("64th")]
  SixtyFourth,
  /// ![128th](https://hedgetechllc.github.io/musicxml/musicxml/datatypes/note-type-128th.png)
  #[rename("128th")]
  OneHundredTwentyEighth,
  /// ![256th](https://hedgetechllc.github.io/musicxml/musicxml/datatypes/note-type-256th.png)
  #[rename("256th")]
  TwoHundredFiftySixth,
  /// ![512th](https://hedgetechllc.github.io/musicxml/musicxml/datatypes/note-type-512th.png)
  #[rename("512th")]
  FiveHundredTwelfth,
  /// ![1024th](https://hedgetechllc.github.io/musicxml/musicxml/datatypes/note-type-1024th.png)
  #[rename("1024th")]
  OneThousandTwentyFourth,
}
