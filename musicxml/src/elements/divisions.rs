use crate::datatypes::PositiveDivisions;
use alloc::string::String;
use musicxml_internal::*;
use musicxml_macros::*;

/// The [Divisions] element indicates how many divisions per quarter note are used to indicate a note's duration.
///
/// For example, if duration = 1 and divisions = 2, this is an eighth note duration. [Duration][super::Duration] and [Divisions]
/// are used directly for generating sound output, so they must be chosen to take tuplets into account.
///
/// Using a [Divisions] element lets us use just one number to represent a duration for each note in the score, while retaining the full power of a
/// fractional representation. If maximum compatibility with Standard MIDI 1.0 files is important, do not have the [Divisions] value exceed 16383.
#[derive(Debug, PartialEq, Eq, ElementDeserialize, ElementSerialize)]
pub struct Divisions {
  /// Element-specific attributes
  pub attributes: (),
  /// Element-specific content
  pub content: PositiveDivisions,
}
