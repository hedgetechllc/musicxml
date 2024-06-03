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

#[derive(Debug, Default, PartialEq, Eq, ContentDeserialize, ContentSerialize)]
pub struct MidiInstrumentContents {
  pub midi_channel: Option<MidiChannel>,
  pub midi_name: Option<MidiName>,
  pub midi_bank: Option<MidiBank>,
  pub midi_program: Option<MidiProgram>,
  pub midi_unpitched: Option<MidiUnpitched>,
  pub volume: Option<Volume>,
  pub pan: Option<Pan>,
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
