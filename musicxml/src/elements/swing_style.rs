use musicxml_internal::*;
use musicxml_macros::*;

/// The [SwingStyle] element is a string describing the style of swing used.
#[derive(Debug, PartialEq, Eq, ElementDeserialize, ElementSerialize)]
#[rename("swing-style")]
pub struct SwingStyle {
  /// Element-specific attributes
  pub attributes: (),
  /// Element-specific content
  pub content: String,
}
