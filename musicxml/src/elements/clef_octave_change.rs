use alloc::string::String;
use musicxml_internal::*;
use musicxml_macros::*;

/// The [ClefOctaveChange] element is used for transposing clefs.
///
/// A treble clef for tenors would have a value of -1.
#[derive(Debug, PartialEq, Eq, ElementDeserialize, ElementSerialize)]
#[rename("clef-octave-change")]
pub struct ClefOctaveChange {
  /// Element-specific attributes
  pub attributes: (),
  /// Element-specific content
  pub content: i8,
}
