use crate::datatypes::Semitones;
use alloc::string::String;
use musicxml_internal::*;
use musicxml_macros::*;

/// The [TuningAlter] element is represented like the [Alter][super::Alter] element, with a different name to reflect its different function in string tuning.
#[derive(Debug, PartialEq, Eq, ElementDeserialize, ElementSerialize)]
#[rename("tuning-alter")]
pub struct TuningAlter {
  /// Element-specific attributes
  pub attributes: (),
  /// Element-specific content
  pub content: Semitones,
}
