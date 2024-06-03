use musicxml_internal::*;
use musicxml_macros::*;

/// The [Humming] element represents a humming voice.
#[derive(Debug, PartialEq, Eq, ElementDeserialize, ElementSerialize)]
pub struct Humming {
  /// Element-specific attributes
  pub attributes: (),
  /// Element-specific content
  pub content: (),
}
