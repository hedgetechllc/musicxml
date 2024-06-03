use musicxml_internal::*;
use musicxml_macros::*;

/// The [HoleShape] element indicates the shape of the hole symbol.
///
/// It is a circle if not specified.
#[derive(Debug, PartialEq, Eq, ElementDeserialize, ElementSerialize)]
#[rename("hole-shape")]
pub struct HoleShape {
  /// Element-specific attributes
  pub attributes: (),
  /// Element-specific content
  pub content: String,
}
