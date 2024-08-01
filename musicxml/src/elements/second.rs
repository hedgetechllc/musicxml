use crate::datatypes::PositiveInteger;
use alloc::string::String;
use musicxml_internal::*;
use musicxml_macros::*;

/// The [Second] element is the part of the swing ratio that refers to the second of two consecutive notes.
#[derive(Debug, PartialEq, Eq, ElementDeserialize, ElementSerialize)]
pub struct Second {
  /// Element-specific attributes
  pub attributes: (),
  /// Element-specific content
  pub content: PositiveInteger,
}
