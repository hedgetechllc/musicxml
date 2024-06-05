use musicxml_internal::*;
use musicxml_macros::*;

/// The [Beats] element indicates the number of beats, as found in the numerator of a time signature.
///
/// ![Beats](https://hedgetechllc.github.io/musicxml/musicxml/elements/beats.png)
#[derive(Debug, PartialEq, Eq, Clone, ElementDeserialize, ElementSerialize)]
pub struct Beats {
  /// Element-specific attributes
  pub attributes: (),
  /// Element-specific content
  pub content: String,
}
