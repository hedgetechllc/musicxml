use musicxml_internal::*;
use musicxml_macros::*;

/// The [Fp] element represents a forte piano dynamic marking.
///
/// ![fp](https://hedgetechllc.github.io/musicxml/musicxml/elements/fp.png)
#[derive(Debug, PartialEq, Eq, ElementDeserialize, ElementSerialize)]
pub struct Fp {
  /// Element-specific attributes
  pub attributes: (),
  /// Element-specific content
  pub content: (),
}
