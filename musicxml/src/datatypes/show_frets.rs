use alloc::string::String;
use musicxml_internal::{DatatypeDeserializer, DatatypeSerializer};
use musicxml_macros::{DatatypeDeserialize, DatatypeSerialize};

/// Indicates whether to show tablature frets as numbers (0, 1, 2) or letters (a, b, c).
///
/// The default choice is numbers.
#[derive(Debug, PartialEq, Eq, DatatypeDeserialize, DatatypeSerialize)]
pub enum ShowFrets {
  /// ![Letters](https://hedgetechllc.github.io/musicxml/musicxml/datatypes/show-frets-letters.png)
  Letters,
  /// ![Numbers](https://hedgetechllc.github.io/musicxml/musicxml/datatypes/show-frets-numbers.png)
  Numbers,
}
