use super::{ActualNotes, NormalDot, NormalNotes, NormalType};
use crate::datatypes::{ShowTuplet, StartStop, YesNo};
use musicxml_internal::*;
use musicxml_macros::*;

/// Attributes pertaining to the [MetronomeTuplet] element.
#[derive(Debug, PartialEq, Eq, AttributeDeserialize, AttributeSerialize)]
pub struct MetronomeTupletAttributes {
  /// Indicates if this is the start or stop of the tuplet.
  pub r#type: StartStop,
  /// Specifies whether or not brackets are put around a symbol for an editorial indication. If not specified, it is left to application defaults.
  pub bracket: Option<YesNo>,
  /// Specifies the number of actual notes in the tuplet.
  pub show_number: Option<ShowTuplet>,
}

/// Contents of the [MetronomeTuplet] element.
#[derive(Debug, PartialEq, Eq, ContentDeserialize, ContentSerialize)]
pub struct MetronomeTupletContents {
  /// The [ActualNotes] element specifies the number of notes in the tuplet.
  pub actual_notes: ActualNotes,
  /// The [NormalNotes] element specifies the normal number of notes in the tuplet.
  pub normal_notes: NormalNotes,
  /// The [NormalType] element specifies the normal type of the tuplet.
  pub normal_type: Option<NormalType>,
  /// The [NormalDot] element specifies the presence of a dot in the tuplet.
  pub normal_dot: Vec<NormalDot>,
}

/// The [MetronomeTuplet] element uses the same element structure as the [TimeModification][super::TimeModification] element,
/// along with some attributes from the [Tuplet][super::Tuplet] element.
///
/// ![MetronomeTuplet](metronome-tuplet.png)
#[derive(Debug, PartialEq, Eq, ElementDeserialize, ElementSerialize)]
#[rename("metronome-tuplet")]
pub struct MetronomeTuplet {
  /// Element-specific attributes
  pub attributes: MetronomeTupletAttributes,
  #[flatten]
  /// Element-specific content
  pub content: MetronomeTupletContents,
}
