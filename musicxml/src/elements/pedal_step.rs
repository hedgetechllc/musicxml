use crate::datatypes::Step;
use musicxml_internal::*;
use musicxml_macros::*;

/// The [PedalStep] element defines the pitch step for a single harp pedal.
#[derive(Debug, PartialEq, Eq, ElementDeserialize, ElementSerialize)]
#[rename("pedal-step")]
pub struct PedalStep {
  /// Element-specific attributes
  pub attributes: (),
  /// Element-specific content
  pub content: Step,
}
