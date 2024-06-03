use musicxml_internal::*;
use musicxml_macros::*;

/// The [MetronomeRelation] element describes the relationship symbol that goes between the two sets of [MetronomeNote][super::MetronomeNote] elements.
///
/// ![MetronomeRelation](metronome-relation.png)
///
/// The currently allowed value is "equals", but this may expand in future versions. If the element is empty, the equals value is used.
#[derive(Debug, PartialEq, Eq, ElementDeserialize, ElementSerialize)]
#[rename("metronome-relation")]
pub struct MetronomeRelation {
  /// Element-specific attributes
  pub attributes: (),
  /// Element-specific content
  pub content: String,
}
