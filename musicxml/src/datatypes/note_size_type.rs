use musicxml_internal::{DatatypeDeserializer, DatatypeSerializer};
use musicxml_macros::{DatatypeDeserialize, DatatypeSerialize};

/// Indicates the type of note size being defined by a [NoteSize][crate::elements::NoteSize] element.
#[derive(Debug, PartialEq, Eq, DatatypeDeserialize, DatatypeSerialize)]
pub enum NoteSizeType {
  /// Used for notes of cue size that do not include a [Grace][crate::elements::Grace] element, whether defined explicitly or implicitly via a [Cue][crate::elements::Cue] element.
  Cue,
  /// Used for notes of cue size that include a [Grace][crate::elements::Grace] element.
  Grace,
  /// Used for notes of grace-cue size.
  #[rename("grace-cue")]
  GraceCue,
  /// Used for notes of large size.
  Large,
}
