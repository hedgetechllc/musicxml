use musicxml_internal::*;
use musicxml_macros::*;

/// The [Sfz] element represents a sforzando sfz dynamic marking.
///
/// ![sfz](sfz.png)
#[derive(Debug, PartialEq, Eq, ElementDeserialize, ElementSerialize)]
pub struct Sfz {
  /// Element-specific attributes
  pub attributes: (),
  /// Element-specific content
  pub content: (),
}
