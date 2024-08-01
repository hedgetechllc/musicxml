use crate::datatypes::Semitones;
use alloc::string::String;
use musicxml_internal::*;
use musicxml_macros::*;

/// The [PedalAlter] element defines the chromatic alteration for a single harp pedal.
#[derive(Debug, PartialEq, Eq, ElementDeserialize, ElementSerialize)]
#[rename("pedal-alter")]
pub struct PedalAlter {
  /// Element-specific attributes
  pub attributes: (),
  /// Element-specific content
  pub content: Semitones,
}
