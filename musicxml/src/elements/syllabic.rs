use musicxml_internal::*;
use musicxml_macros::*;

/// The [Syllabic] element indicates lyric hyphenation.
///
/// The single, begin, end, and middle values represent single-syllable words, word-beginning syllables, word-ending syllables, and mid-word syllables, respectively.
#[derive(Debug, PartialEq, Eq, ElementDeserialize, ElementSerialize)]
pub struct Syllabic {
  /// Element-specific attributes
  pub attributes: (),
  /// Element-specific content
  pub content: crate::datatypes::Syllabic,
}
