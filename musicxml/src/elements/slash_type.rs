use crate::datatypes::NoteTypeValue;
use alloc::string::String;
use musicxml_internal::*;
use musicxml_macros::*;

/// The [SlashType] element indicates the graphical note type to use for the display of repetition marks.
#[derive(Debug, PartialEq, Eq, ElementDeserialize, ElementSerialize)]
#[rename("slash-type")]
pub struct SlashType {
  /// Element-specific attributes
  pub attributes: (),
  /// Element-specific content
  pub content: NoteTypeValue,
}
