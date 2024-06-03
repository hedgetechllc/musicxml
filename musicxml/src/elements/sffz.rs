use musicxml_internal::*;
use musicxml_macros::*;

/// The [Sffz] element represents a sforzando sffz dynamic marking.
///
/// ![sffz](sffz.png)
#[derive(Debug, PartialEq, Eq, ElementDeserialize, ElementSerialize)]
pub struct Sffz {
  /// Element-specific attributes
  pub attributes: (),
  /// Element-specific content
  pub content: (),
}
