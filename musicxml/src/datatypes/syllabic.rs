use musicxml_internal::{DatatypeDeserializer, DatatypeSerializer};
use musicxml_macros::{DatatypeDeserialize, DatatypeSerialize};

/// Indicates lyric hyphenation based on the syllabic type.
///
/// The single, begin, end, and middle values represent single-syllable words,
/// word-beginning syllables, word-ending syllables, and mid-word syllables, respectively.
#[derive(Debug, PartialEq, Eq, DatatypeDeserialize, DatatypeSerialize)]
pub enum Syllabic {
  /// ![Begin](https://hedgetechllc.github.io/musicxml/musicxml/datatypes/syllabic-begin.png)
  Begin,
  /// ![End](https://hedgetechllc.github.io/musicxml/musicxml/datatypes/syllabic-end.png)
  End,
  /// ![Middle](https://hedgetechllc.github.io/musicxml/musicxml/datatypes/syllabic-middle.png)
  Middle,
  /// ![Single](https://hedgetechllc.github.io/musicxml/musicxml/datatypes/syllabic-single.png)
  Single,
}
