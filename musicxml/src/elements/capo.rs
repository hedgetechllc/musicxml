use crate::datatypes::NonNegativeInteger;
use musicxml_internal::*;
use musicxml_macros::*;

/// The [Capo] element indicates at which fret a capo should be placed on a fretted instrument.
/// 
/// This changes the open tuning of the strings specified by the [StaffTuning][super::StaffTuning] element by the specified number of half-steps.
#[derive(Debug, PartialEq, Eq, ElementDeserialize, ElementSerialize)]
pub struct Capo {
  /// Element-specific attributes
  pub attributes: (),
  /// Element-specific content
  pub content: NonNegativeInteger,
}
