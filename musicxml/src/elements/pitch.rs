use super::{Alter, Octave, Step};
use musicxml_internal::*;
use musicxml_macros::*;

#[derive(Debug, PartialEq, Eq, ContentDeserialize, ContentSerialize)]
pub struct PitchContents {
  pub step: Step,
  pub alter: Option<Alter>,
  pub octave: Octave,
}

/// [Pitch] is represented as a combination of the step of the diatonic scale, the chromatic alteration, and the octave.
#[derive(Debug, PartialEq, Eq, ElementDeserialize, ElementSerialize)]
pub struct Pitch {
  /// Element-specific attributes
  pub attributes: (),
  #[flatten]
  /// Element-specific content
  pub content: PitchContents,
}
