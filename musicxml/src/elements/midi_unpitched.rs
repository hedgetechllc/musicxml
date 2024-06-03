use crate::datatypes::Midi128;
use musicxml_internal::*;
use musicxml_macros::*;

/// For unpitched instruments, the [MidiUnpitched] element specifies a MIDI 1.0 note number ranging from 1 to 128.
///
/// It is usually used with MIDI banks for percussion. Note that MIDI 1.0 note numbers are generally specified from 0 to 127
/// rather than the 1 to 128 numbering used in this element.
#[derive(Debug, PartialEq, Eq, ElementDeserialize, ElementSerialize)]
#[rename("midi-unpitched")]
pub struct MidiUnpitched {
  /// Element-specific attributes
  pub attributes: (),
  /// Element-specific content
  pub content: Midi128,
}
