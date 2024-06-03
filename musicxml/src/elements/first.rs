use crate::datatypes::PositiveInteger;
use musicxml_internal::*;
use musicxml_macros::*;

/// The [First] element is the part of the swing ratio that refers to the first of two consecutive notes.
#[derive(Debug, PartialEq, Eq, ElementDeserialize, ElementSerialize)]
pub struct First {
  /// Element-specific attributes
  pub attributes: (),
  /// Element-specific content
  pub content: PositiveInteger,
}
