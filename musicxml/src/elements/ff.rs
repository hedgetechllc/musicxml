use musicxml_internal::*;
use musicxml_macros::*;

/// The [Ff] element represents a fortissimo dynamic marking.
/// 
/// ![ff](ff.png)
#[derive(Debug, PartialEq, Eq, ElementDeserialize, ElementSerialize)]
pub struct Ff {
  /// Element-specific attributes
  pub attributes: (),
  /// Element-specific content
  pub content: (),
}
