use crate::datatypes::NoteTypeValue;
use musicxml_internal::*;
use musicxml_macros::*;

/// The [BeatUnit] element indicates the graphical note type to use in a metronome mark.
#[derive(Debug, PartialEq, Eq, ElementDeserialize, ElementSerialize)]
#[rename("beat-unit")]
pub struct BeatUnit {
  /// Element-specific attributes
  pub attributes: (),
  /// Element-specific content
  pub content: NoteTypeValue,
}
