use musicxml_internal::{DatatypeDeserializer, DatatypeSerializer};
use musicxml_macros::{DatatypeDeserialize, DatatypeSerialize};

/// Indicates lyric hyphenation based on the syllabic type.
///
/// The single, begin, end, and middle values represent single-syllable words,
/// word-beginning syllables, word-ending syllables, and mid-word syllables, respectively.
#[derive(Debug, PartialEq, Eq, DatatypeDeserialize, DatatypeSerialize)]
pub enum Syllabic {
  /// ![Begin](syllabic-begin.png)
  Begin,
  /// ![End](syllabic-end.png)
  End,
  /// ![Middle](syllabic-middle.png)
  Middle,
  /// ![Single](syllabic-single.png)
  Single,
}
