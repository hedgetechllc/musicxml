use crate::datatypes::NoteTypeValue;
use musicxml_internal::*;
use musicxml_macros::*;

/// The [NormalType] element specifies note durations that are altered by inclusion in a tuplet.
/// 
/// If the type associated with the number in the [NormalNotes][super::NormalNotes] element is different than the current note type
/// (e.g., a quarter note within an eighth note triplet), then the [NormalNotes][super::NormalNotes] type
/// (e.g. eighth) is specified in the [NormalType] and [NormalDot][super::NormalDot] elements.
#[derive(Debug, PartialEq, Eq, ElementDeserialize, ElementSerialize)]
#[rename("normal-type")]
pub struct NormalType {
  /// Element-specific attributes
  pub attributes: (),
  /// Element-specific content
  pub content: NoteTypeValue,
}
