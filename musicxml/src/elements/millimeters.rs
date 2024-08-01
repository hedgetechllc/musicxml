use alloc::string::String;
use musicxml_internal::*;
use musicxml_macros::*;

/// The [Millimeters] element contains the number of millimeters that correspond to the given number of tenths within the [Scaling][super::Scaling] element formula.
#[derive(Debug, PartialEq, Eq, ElementDeserialize, ElementSerialize)]
pub struct Millimeters {
  /// Element-specific attributes
  pub attributes: (),
  /// Element-specific content
  pub content: crate::datatypes::Millimeters,
}
