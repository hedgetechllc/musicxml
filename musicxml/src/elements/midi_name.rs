use alloc::string::String;
use musicxml_internal::*;
use musicxml_macros::*;

/// The [MidiName] element corresponds to a `ProgramName` meta-event within a Standard MIDI File.
#[derive(Debug, PartialEq, Eq, ElementDeserialize, ElementSerialize)]
#[rename("midi-name")]
pub struct MidiName {
  /// Element-specific attributes
  pub attributes: (),
  /// Element-specific content
  pub content: String,
}
