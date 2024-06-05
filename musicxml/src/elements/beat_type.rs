use musicxml_internal::*;
use musicxml_macros::*;

/// The [BeatType] element indicates the beat unit, as found in the denominator of a time signature.
///
/// ![BeatType](https://hedgetechllc.github.io/musicxml/musicxml/elements/beat-type.png)
#[derive(Debug, PartialEq, Eq, ElementDeserialize, ElementSerialize)]
#[rename("beat-type")]
pub struct BeatType {
  /// Element-specific attributes
  pub attributes: (),
  /// Element-specific content
  pub content: String,
}
