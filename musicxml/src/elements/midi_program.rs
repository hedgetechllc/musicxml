use crate::datatypes::Midi128;
use musicxml_internal::*;
use musicxml_macros::*;

/// The [MidiProgram] element specifies a MIDI 1.0 program number ranging from 1 to 128.
#[derive(Debug, PartialEq, Eq, ElementDeserialize, ElementSerialize)]
#[rename("midi-program")]
pub struct MidiProgram {
  /// Element-specific attributes
  pub attributes: (),
  /// Element-specific content
  pub content: Midi128,
}
