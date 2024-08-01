use crate::datatypes::NonNegativeInteger;
use alloc::string::String;
use musicxml_internal::*;
use musicxml_macros::*;

/// The [NormalNotes] element specifies note durations that are altered by inclusion in a tuplet.
///
/// If the type associated with the number in the [NormalNotes] element is different than the current note type
/// (e.g., a quarter note within an eighth note triplet), then the [NormalNotes] type (e.g. eighth) is specified
/// in the [NormalType][super::NormalType] and [NormalDot][super::NormalDot] elements.
#[derive(Debug, PartialEq, Eq, ElementDeserialize, ElementSerialize)]
#[rename("normal-notes")]
pub struct NormalNotes {
  /// Element-specific attributes
  pub attributes: (),
  /// Element-specific content
  pub content: NonNegativeInteger,
}
