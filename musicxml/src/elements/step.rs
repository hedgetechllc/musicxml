use alloc::string::String;
use musicxml_internal::*;
use musicxml_macros::*;

/// The [Step] element represents a step of the diatonic scale, represented using the English letters A through G.
#[derive(Debug, PartialEq, Eq, ElementDeserialize, ElementSerialize)]
pub struct Step {
  /// Element-specific attributes
  pub attributes: (),
  /// Element-specific content
  pub content: crate::datatypes::Step,
}
