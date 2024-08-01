use super::{ActualNotes, NormalDot, NormalNotes, NormalType};
use alloc::{string::String, vec::Vec};
use musicxml_internal::*;
use musicxml_macros::*;

/// Contents of the [TimeModification] element.
#[derive(Debug, PartialEq, Eq, ContentDeserialize, ContentSerialize)]
pub struct TimeModificationContents {
  /// The [ActualNotes] element specifies the number of notes in the tuplet.
  pub actual_notes: ActualNotes,
  /// The [NormalNotes] element specifies the number of notes in the normal, in-effect duration.
  pub normal_notes: NormalNotes,
  /// The [NormalType] element specifies the normal, in-effect note type.
  pub normal_type: Option<NormalType>,
  /// The [NormalDot] element specifies the normal, in-effect dot.
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
