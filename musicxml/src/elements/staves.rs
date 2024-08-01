use crate::datatypes::NonNegativeInteger;
use alloc::string::String;
use musicxml_internal::*;
use musicxml_macros::*;

/// The [Staves] element is used if there is more than one staff represented in the given part (e.g., 2 staves for typical piano parts).
///
/// If absent, a value of 1 is assumed. Staves are ordered from top to bottom in a part in numerical order, with staff 1 above staff 2.
#[derive(Debug, PartialEq, Eq, ElementDeserialize, ElementSerialize)]
pub struct Staves {
  /// Element-specific attributes
  pub attributes: (),
  /// Element-specific content
  pub content: NonNegativeInteger,
}
