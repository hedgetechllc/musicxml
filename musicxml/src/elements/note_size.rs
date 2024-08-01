use crate::datatypes::{NonNegativeDecimal, NoteSizeType};
use alloc::{string::String, vec::Vec};
use musicxml_internal::*;
use musicxml_macros::*;

/// Attributes pertaining to the [NoteSize] element.
#[derive(Debug, PartialEq, Eq, AttributeDeserialize, AttributeSerialize)]
pub struct NoteSizeAttributes {
  /// The type of note size being defined.
  pub r#type: NoteSizeType,
}

/// The [NoteSize] element indicates the numeric percentage of the regular note size to use for notes with cue and large size,
/// as defined in the [Type][super::Type] element.
///
/// A value of 100 would be identical to the size of a regular note as defined by the music font.
#[derive(Debug, PartialEq, Eq, ElementDeserialize, ElementSerialize)]
#[rename("note-size")]
pub struct NoteSize {
  /// Element-specific attributes
  pub attributes: NoteSizeAttributes,
  /// Element-specific content
  pub content: NonNegativeDecimal,
}
