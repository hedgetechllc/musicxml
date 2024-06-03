use super::{ActualNotes, NormalDot, NormalNotes, NormalType};
use musicxml_internal::*;
use musicxml_macros::*;

#[derive(Debug, PartialEq, Eq, ContentDeserialize, ContentSerialize)]
pub struct TimeModificationContents {
  pub actual_notes: ActualNotes,
  pub normal_notes: NormalNotes,
  pub normal_type: Option<NormalType>,
  pub normal_dot: Vec<NormalDot>,
}

/// [TimeModification] indicates tuplets, double-note tremolos, and other durational changes.
///
/// A [TimeModification] element shows how the cumulative, sounding effect of tuplets and double-note tremolos compare to the written
/// note type represented by the [Type][super::Type] and [Dot][super::Dot] elements. Nested tuplets and other notations that use more detailed
/// information need both the [TimeModification] and [Tuplet][super::Tuplet] elements to be represented accurately.
#[derive(Debug, PartialEq, Eq, ElementDeserialize, ElementSerialize)]
#[rename("time-modification")]
pub struct TimeModification {
  /// Element-specific attributes
  pub attributes: (),
  #[flatten]
  /// Element-specific content
  pub content: TimeModificationContents,
}
