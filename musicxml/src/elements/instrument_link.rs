use crate::datatypes::IdRef;
use musicxml_internal::*;
use musicxml_macros::*;

/// Attributes pertaining to the [InstrumentLink] element.
#[derive(Debug, PartialEq, Eq, AttributeDeserialize, AttributeSerialize)]
pub struct InstrumentLinkAttributes {
  /// Specifies an ID that is unique to the entire document.
  pub id: IdRef,
}

/// Multiple [PartLink][super::PartLink] elements can link a condensed part within a score file to multiple MusicXML parts files.
/// 
/// For example, a "Clarinet 1 and 2" part in a score file could link to separate "Clarinet 1" and "Clarinet 2" part files.
/// The [InstrumentLink] element distinguishes which of the [ScoreInstruments][super::ScoreInstrument] within a [ScorePart][super::ScorePart]
/// are in which part file.
#[derive(Debug, PartialEq, Eq, ElementDeserialize, ElementSerialize)]
#[rename("instrument-link")]
pub struct InstrumentLink {
  /// Element-specific attributes
  pub attributes: InstrumentLinkAttributes,
  /// Element-specific content
  pub content: (),
}
