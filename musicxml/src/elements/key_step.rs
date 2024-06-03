use crate::datatypes::Step;
use musicxml_internal::*;
use musicxml_macros::*;

/// The [KeyStep] element indicates the pitch step to be altered, represented using the same names as in the [Step][super::Step] element.
///
/// The different element names indicate the different meaning of altering notes in a scale versus altering a sounding pitch.
#[derive(Debug, PartialEq, Eq, ElementDeserialize, ElementSerialize)]
#[rename("key-step")]
pub struct KeyStep {
  /// Element-specific attributes
  pub attributes: (),
  /// Element-specific content
  pub content: Step,
}
