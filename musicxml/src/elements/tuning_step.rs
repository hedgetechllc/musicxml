use crate::datatypes::Step;
use musicxml_internal::*;
use musicxml_macros::*;

/// The [TuningStep] element is represented like the [Step][super::Step] element, with a different name to reflect its different function in string tuning.
#[derive(Debug, PartialEq, Eq, ElementDeserialize, ElementSerialize)]
#[rename("tuning-step")]
pub struct TuningStep {
  /// Element-specific attributes
  pub attributes: (),
  /// Element-specific content
  pub content: Step,
}
