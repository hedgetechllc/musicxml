use super::{MetronomeBeam, MetronomeDot, MetronomeTied, MetronomeTuplet, MetronomeType};
use musicxml_internal::*;
use musicxml_macros::*;

#[derive(Debug, PartialEq, Eq, ContentDeserialize, ContentSerialize)]
pub struct MetronomeNoteContents {
  pub metronome_type: MetronomeType,
  pub metronome_dot: Vec<MetronomeDot>,
  pub metronome_beam: Vec<MetronomeBeam>,
  pub metronome_tied: Option<MetronomeTied>,
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
