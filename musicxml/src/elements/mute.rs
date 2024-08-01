use alloc::string::String;
use musicxml_internal::*;
use musicxml_macros::*;

/// The [Mute] element represents muting playback for different instruments, including brass, winds, and strings.
#[derive(Debug, PartialEq, Eq, ElementDeserialize, ElementSerialize)]
pub struct Mute {
  /// Element-specific attributes
  pub attributes: (),
  /// Element-specific content
  pub content: crate::datatypes::Mute,
}
