use crate::datatypes::{IdRef, Midi16};
use musicxml_internal::*;
use musicxml_macros::*;

/// Attributes pertaining to the [MidiDevice] element.
#[derive(Debug, Default, PartialEq, Eq, AttributeDeserialize, AttributeSerialize)]
pub struct MidiDeviceAttributes {
  /// Specifies an ID that is unique to the entire document.
  pub id: Option<IdRef>,
  /// A number from 1 to 16 that can be used with the unofficial MIDI 1.0 port (or cable) meta event.
  pub port: Option<Midi16>,
}

/// The [MidiDevice] element corresponds to the `DeviceName` meta event in Standard MIDI Files.
///
/// Unlike the `DeviceName` meta event, there can be multiple [MidiDevice] elements per MusicXML part.
#[derive(Debug, PartialEq, Eq, ElementDeserialize, ElementSerialize)]
#[rename("midi-device")]
pub struct MidiDevice {
  /// Element-specific attributes
  pub attributes: MidiDeviceAttributes,
  /// Element-specific content
  pub content: String,
}
