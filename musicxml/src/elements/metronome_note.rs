use super::{MetronomeBeam, MetronomeDot, MetronomeTied, MetronomeTuplet, MetronomeType};
use musicxml_internal::*;
use musicxml_macros::*;

/// Contents of the [MetronomeNote] element.
#[derive(Debug, PartialEq, Eq, ContentDeserialize, ContentSerialize)]
pub struct MetronomeNoteContents {
  /// The [MetronomeType] element specifies the note type for the metronome mark.
  pub metronome_type: MetronomeType,
  /// The [MetronomeDot] element specifies the presence of a dot in the metronome mark.
  pub metronome_dot: Vec<MetronomeDot>,
  /// The [MetronomeBeam] element specifies the presence of a beam in the metronome mark.
  pub metronome_beam: Vec<MetronomeBeam>,
  /// The [MetronomeTied] element specifies the presence of a tie in the metronome mark.
  pub metronome_tied: Option<MetronomeTied>,
  /// The [MetronomeTuplet] element specifies the presence of a tuplet in the metronome mark.
  pub metronome_tuplet: Option<MetronomeTuplet>,
}

/// The [MetronomeNote] element defines the appearance of a note within a metric relationship mark.
#[derive(Debug, PartialEq, Eq, ElementDeserialize, ElementSerialize)]
#[rename("metronome-note")]
pub struct MetronomeNote {
  /// Element-specific attributes
  pub attributes: (),
  #[flatten]
  /// Element-specific content
  pub content: MetronomeNoteContents,
}
