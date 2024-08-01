use alloc::string::String;
use musicxml_internal::*;
use musicxml_macros::*;

/// The [Laughing] element represents a laughing voice.
#[derive(Debug, PartialEq, Eq, ElementDeserialize, ElementSerialize)]
pub struct Laughing {
  /// Element-specific attributes
  pub attributes: (),
  /// Element-specific content
  pub content: (),
}
