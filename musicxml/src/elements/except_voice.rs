use alloc::string::String;
use musicxml_internal::*;
use musicxml_macros::*;

/// The [ExceptVoice] element is used to specify a combination of slash notation and regular notation.
///
/// Any [Note][super::Note] elements that are in voices specified by the [ExceptVoice] elements are displayed in normal notation,
/// in addition to the slash notation that is always displayed.
#[derive(Debug, PartialEq, Eq, ElementDeserialize, ElementSerialize)]
#[rename("except-voice")]
pub struct ExceptVoice {
  /// Element-specific attributes
  pub attributes: (),
  /// Element-specific content
  pub content: String,
}
