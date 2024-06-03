use super::{Ensemble, InstrumentAbbreviation, InstrumentName, InstrumentSound, Solo, VirtualInstrument};
use crate::datatypes::Id;
use musicxml_internal::*;
use musicxml_macros::*;

/// Attributes pertaining to the [ScoreInstrument] element.
#[derive(Debug, PartialEq, Eq, AttributeDeserialize, AttributeSerialize)]
pub struct ScoreInstrumentAttributes {
  /// Specifies an ID that is unique to the entire document.
  pub id: Id,
}

/// Contents of the [ScoreInstrument] element.
#[derive(Debug, PartialEq, Eq, ContentDeserialize, ContentSerialize)]
pub struct ScoreInstrumentContents {
  /// The [InstrumentName] element specifies the name of the instrument.
  pub instrument_name: InstrumentName,
  /// The [InstrumentAbbreviation] element specifies the abbreviation for the instrument name.
  pub instrument_abbreviation: Option<InstrumentAbbreviation>,
  /// The [InstrumentSound] element specifies the sound of the instrument.
  pub instrument_sound: Option<InstrumentSound>,
  /// The [Solo] element specifies the solo instrument.
  pub solo: Option<Solo>,
  /// The [Ensemble] element specifies the ensemble instrument.
  pub ensemble: Option<Ensemble>,
  /// The [VirtualInstrument] element specifies the virtual instrument.
  pub virtual_instrument: Option<VirtualInstrument>,
}

/// The [ScoreInstrument] element represents a single instrument within a [ScorePart][super::ScorePart].
///
/// As with the [ScorePart][super::ScorePart] element, each [ScoreInstrument] has a required ID attribute, a name, and an optional abbreviation.
///
/// A [ScoreInstrument] element is also required if the score specifies MIDI 1.0 channels, banks, or programs. An initial
/// [MidiInstrument][super::MidiInstrument] assignment can also be made here. MusicXML software should be able to automatically assign reasonable channels and
/// instruments without these elements in simple cases, such as where part names match General MIDI instrument names.
///
/// The [ScoreInstrument] element can also distinguish multiple instruments of the same type that are on the same part, such as Clarinet 1 and
/// Clarinet 2 instruments within a Clarinets 1 and 2 part.
#[derive(Debug, PartialEq, Eq, ElementDeserialize, ElementSerialize)]
#[rename("score-instrument")]
pub struct ScoreInstrument {
  /// Element-specific attributes
  pub attributes: ScoreInstrumentAttributes,
  #[flatten]
  /// Element-specific content
  pub content: ScoreInstrumentContents,
}
