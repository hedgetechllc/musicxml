use musicxml_internal::*;
use musicxml_macros::*;

/// The content of the [HoleType] element indicates what the hole symbol represents in terms of instrument fingering or other techniques.
#[derive(Debug, PartialEq, Eq, ElementDeserialize, ElementSerialize)]
#[rename("hole-type")]
pub struct HoleType {
  /// Element-specific attributes
  pub attributes: (),
  /// Element-specific content
  pub content: String,
}
