use super::{Alter, Octave, Step};
use alloc::{string::String, vec::Vec};
use musicxml_internal::*;
use musicxml_macros::*;

/// Contents of the [Pitch] element.
#[derive(Debug, PartialEq, Eq, ContentDeserialize, ContentSerialize)]
pub struct PitchContents {
  /// The [Step] element represents the step of the diatonic scale.
  pub step: Step,
  /// The [Alter] element represents the chromatic alteration.
  pub alter: Option<Alter>,
  /// The [Octave] element represents the octave.
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
