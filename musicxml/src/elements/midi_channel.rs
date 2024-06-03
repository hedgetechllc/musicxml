use crate::datatypes::Midi16;
use musicxml_internal::*;
use musicxml_macros::*;

/// The [MidiChannel] element specifies a MIDI 1.0 channel numbers ranging from 1 to 16.
#[derive(Debug, PartialEq, Eq, ElementDeserialize, ElementSerialize)]
#[rename("midi-channel")]
pub struct MidiChannel {
  /// Element-specific attributes
  pub attributes: (),
  /// Element-specific content
  pub content: Midi16,
}
