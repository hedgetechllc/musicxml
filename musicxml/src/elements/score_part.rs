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

/// Contents of the [ScorePart] element.
#[derive(Debug, PartialEq, Eq, ContentDeserialize, ContentSerialize)]
pub struct ScorePartContents {
  /// The [Identification] element contains information about the creator, rights, and encoding of the part.
  pub identification: Option<Identification>,
  /// The [PartLink] element specifies a link to a part group within a score.
  pub part_link: Vec<PartLink>,
  /// The [PartName] element specifies the name of the part.
  pub part_name: PartName,
  /// The [PartNameDisplay] element specifies how the part name should be displayed.
  pub part_name_display: Option<PartNameDisplay>,
  /// The [PartAbbreviation] element specifies the abbreviation for the part name.
  pub part_abbreviation: Option<PartAbbreviation>,
  /// The [PartAbbreviationDisplay] element specifies how the part abbreviation should be displayed.
  pub part_abbreviation_display: Option<PartAbbreviationDisplay>,
  /// The [Group] element allows for multiple parts within a score to be grouped together for a variety of purposes.
  pub group: Vec<Group>,
  /// The [ScoreInstrument] element represents a single instrument within a [ScorePart].
  pub score_instrument: Vec<ScoreInstrument>,
  /// The [Player] element allows for multiple players per [ScorePart] for use in listening applications.
  pub player: Vec<Player>,
  /// The [MidiDevice] element specifies MIDI 1.0 device settings.
  pub midi_device: Vec<MidiDevice>,
  /// The [MidiInstrument] element defines MIDI 1.0 instrument playback.
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
