use super::{Elevation, MidiBank, MidiChannel, MidiName, MidiProgram, MidiUnpitched, Pan, Volume};
use crate::datatypes::IdRef;
use musicxml_internal::*;
use musicxml_macros::*;

/// Attributes pertaining to the [MidiInstrument] element.
#[derive(Debug, PartialEq, Eq, AttributeDeserialize, AttributeSerialize)]
pub struct MidiInstrumentAttributes {
  /// Specifies an ID that is unique to the entire document.
  pub id: IdRef,
}

/// Contents of the [MidiInstrument] element.
#[derive(Debug, Default, PartialEq, Eq, ContentDeserialize, ContentSerialize)]
pub struct MidiInstrumentContents {
  /// The [MidiChannel] element specifies the MIDI 1.0 channel number.
  pub midi_channel: Option<MidiChannel>,
  /// The [MidiName] element specifies the MIDI 1.0 instrument name.
  pub midi_name: Option<MidiName>,
  /// The [MidiBank] element specifies the MIDI 1.0 bank number.
  pub midi_bank: Option<MidiBank>,
  /// The [MidiProgram] element specifies the MIDI 1.0 program number.
  pub midi_program: Option<MidiProgram>,
  /// The [MidiUnpitched] element specifies the MIDI 1.0 unpitched percussion MIDI key.
  pub midi_unpitched: Option<MidiUnpitched>,
  /// The [Volume] element specifies the MIDI 1.0 volume.
  pub volume: Option<Volume>,
  /// The [Pan] element specifies the MIDI 1.0 pan.
  pub pan: Option<Pan>,
  /// The [Elevation] element specifies the MIDI 1.0 elevation.
  pub elevation: Option<Elevation>,
}

/// The [MidiInstrument] element defines MIDI 1.0 instrument playback.
///
/// The [MidiInstrument] element can be a part of either the [ScoreInstrument][super::ScoreInstrument] element at the start of a part,
/// or the [Sound][super::Sound] element within a part.
#[derive(Debug, PartialEq, Eq, ElementDeserialize, ElementSerialize)]
#[rename("midi-instrument")]
pub struct MidiInstrument {
  /// Element-specific attributes
  pub attributes: MidiInstrumentAttributes,
  #[flatten]
  /// Element-specific content
  pub content: MidiInstrumentContents,
}
