use crate::datatypes::NoteTypeValue;
use musicxml_internal::*;
use musicxml_macros::*;

/// The [MetronomeType] element works like the [Type][super::Type] element in defining metric relationships.
#[derive(Debug, PartialEq, Eq, ElementDeserialize, ElementSerialize)]
#[rename("metronome-type")]
pub struct MetronomeType {
  /// Element-specific attributes
  pub attributes: (),
  /// Element-specific content
  pub content: NoteTypeValue,
}
