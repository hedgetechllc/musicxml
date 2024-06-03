use super::{
  Group, Identification, MidiDevice, MidiInstrument, PartAbbreviation, PartAbbreviationDisplay, PartLink, PartName,
  PartNameDisplay, Player, ScoreInstrument,
};
use crate::datatypes::Id;
use musicxml_internal::*;
use musicxml_macros::*;

/// Attributes pertaining to the [ScorePart] element.
#[derive(Debug, PartialEq, Eq, AttributeDeserialize, AttributeSerialize)]
pub struct ScorePartAttributes {
  /// Specifies an ID that is unique to the entire document.
  pub id: Id,
}

#[derive(Debug, PartialEq, Eq, ContentDeserialize, ContentSerialize)]
pub struct ScorePartContents {
  pub identification: Option<Identification>,
  pub part_linke: Vec<PartLink>,
  pub part_name: PartName,
  pub part_name_display: Option<PartNameDisplay>,
  pub part_abbreviation: Option<PartAbbreviation>,
  pub part_abbreviation_display: Option<PartAbbreviationDisplay>,
  pub group: Vec<Group>,
  pub score_instrument: Vec<ScoreInstrument>,
  pub player: Vec<Player>,
  pub midi_device: Vec<MidiDevice>,
  pub midi_instrument: Vec<MidiInstrument>,
}

/// The [ScorePart] element collects part-wide information for each part in a score.
/// 
/// Often each MusicXML part corresponds to a track in a Standard MIDI Format 1 file. In this case, the [MidiDevice] element is used to make a
/// MIDI device or port assignment for the given track or specific MIDI instruments. Initial [MidiInstrument] assignments may be made here as well.
#[derive(Debug, PartialEq, Eq, ElementDeserialize, ElementSerialize)]
#[rename("score-part")]
pub struct ScorePart {
  /// Element-specific attributes
  pub attributes: ScorePartAttributes,
  #[flatten]
  /// Element-specific content
  pub content: ScorePartContents,
}
