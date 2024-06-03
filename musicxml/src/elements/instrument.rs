use crate::datatypes::IdRef;
use musicxml_internal::*;
use musicxml_macros::*;

/// Attributes pertaining to the [Instrument] element.
#[derive(Debug, PartialEq, Eq, AttributeDeserialize, AttributeSerialize)]
pub struct InstrumentAttributes {
  /// Specifies an ID that is unique to the entire document.
  pub id: IdRef,
}

/// The [Instrument] element distinguishes between [ScoreInstrument][super::ScoreInstrument] elements in a [ScorePart][super::ScorePart].
///
/// If multiple [ScoreInstrument][super::ScoreInstrument] elements are specified in a [ScorePart][super::ScorePart], there should be an
/// [Instrument] element for each note in the [Part][super::Part]. Notes that are shared between multiple [ScoreInstruments][super::ScoreInstrument]
/// can have more than one [Instrument] element.
#[derive(Debug, PartialEq, Eq, ElementDeserialize, ElementSerialize)]
pub struct Instrument {
  /// Element-specific attributes
  pub attributes: InstrumentAttributes,
  /// Element-specific content
  pub content: (),
}
