use crate::datatypes::Midi16384;
use musicxml_internal::*;
use musicxml_macros::*;

/// The [MidiBank] element specifies a MIDI 1.0 bank number ranging from 1 to 16,384.
#[derive(Debug, PartialEq, Eq, ElementDeserialize, ElementSerialize)]
#[rename("midi-bank")]
pub struct MidiBank {
  /// Element-specific attributes
  pub attributes: (),
  /// Element-specific content
  pub content: Midi16384,
}
