use super::{PedalAlter, PedalStep};
use alloc::{string::String, vec::Vec};
use musicxml_internal::*;
use musicxml_macros::*;

/// Contents of the [PedalTuning] element.
#[derive(Debug, PartialEq, Eq, ContentDeserialize, ContentSerialize)]
pub struct PedalTuningContents {
  /// The [PedalStep] element specifies the step of the tuning.
  pub pedal_step: PedalStep,
  /// The [PedalAlter] element specifies the alteration of the tuning.
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
