use alloc::string::String;
use musicxml_internal::*;
use musicxml_macros::*;

/// The [SlashDot] element is used to specify any augmentation dots in the note type used to display repetition marks.
#[derive(Debug, PartialEq, Eq, ElementDeserialize, ElementSerialize)]
#[rename("slash-dot")]
pub struct SlashDot {
  /// Element-specific attributes
  pub attributes: (),
  /// Element-specific content
  pub content: (),
}
