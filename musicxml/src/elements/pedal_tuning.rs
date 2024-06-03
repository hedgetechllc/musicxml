use super::{PedalAlter, PedalStep};
use musicxml_internal::*;
use musicxml_macros::*;

#[derive(Debug, PartialEq, Eq, ContentDeserialize, ContentSerialize)]
pub struct PedalTuningContents {
  pub pedal_step: PedalStep,
  pub pedal_alter: PedalAlter,
}

/// The [PedalTuning] element specifies the tuning of a single harp pedal.
#[derive(Debug, PartialEq, Eq, ElementDeserialize, ElementSerialize)]
#[rename("pedal-tuning")]
pub struct PedalTuning {
  /// Element-specific attributes
  pub attributes: (),
  #[flatten]
  /// Element-specific content
  pub content: PedalTuningContents,
}
