use crate::datatypes::Semitones;
use alloc::string::String;
use musicxml_internal::*;
use musicxml_macros::*;

/// The [BendAlter] element indicates the number of semitones in the bend, similar to the [Alter][super::Alter] element.
///
/// As with the [Alter][super::Alter] element, numbers like 0.5 can be used to indicate microtones. Negative values indicate pre-bends or releases.
/// The [PreBend][super::PreBend] and [Release][super::Release] elements are used to distinguish what is intended.
/// Because the [BendAlter] element represents the number of steps in the bend, a release after a bend has a negative [BendAlter] value, not a zero value.
#[derive(Debug, PartialEq, Eq, ElementDeserialize, ElementSerialize)]
#[rename("bend-alter")]
pub struct BendAlter {
  /// Element-specific attributes
  pub attributes: (),
  /// Element-specific content
  pub content: Semitones,
}
