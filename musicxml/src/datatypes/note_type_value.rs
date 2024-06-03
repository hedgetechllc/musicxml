use musicxml_internal::{DatatypeDeserializer, DatatypeSerializer};
use musicxml_macros::{DatatypeDeserialize, DatatypeSerialize};

/// Used for the MusicXML [Type][crate::elements::Type] element and represents the graphic note type,
/// from 1024th (shortest) to maxima (longest).
#[derive(Debug, PartialEq, Eq, DatatypeDeserialize, DatatypeSerialize)]
pub enum NoteTypeValue {
  /// ![Maxima](note-type-maxima.png)
  Maxima,
  /// ![Long](note-type-long.png)
  Long,
  /// ![Breve](note-type-breve.png)
  Breve,
  /// ![Whole](note-type-whole.png)
  Whole,
  /// ![Half](note-type-half.png)
  Half,
  /// ![Quarter](note-type-quarter.png)
  Quarter,
  /// ![Eighth](note-type-eighth.png)
  Eighth,
  /// ![16th](note-type-16th.png)
  #[rename("16th")]
  Sixteenth,
  /// ![32nd](note-type-32nd.png)
  #[rename("32th")]
  ThirtySecond,
  /// ![64th](note-type-64th.png)
  #[rename("64th")]
  SixtyFourth,
  /// ![128th](note-type-128th.png)
  #[rename("128th")]
  OneHundredTwentyEighth,
  /// ![256th](note-type-256th.png)
  #[rename("256th")]
  TwoHundredFiftySixth,
  /// ![512th](note-type-512th.png)
  #[rename("512th")]
  FiveHundredTwelfth,
  /// ![1024th](note-type-1024th.png)
  #[rename("1024th")]
  OneThousandTwentyFourth,
}
