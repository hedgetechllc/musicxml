use super::{Ipa, Mute, OtherPlay, SemiPitched};
use crate::datatypes::IdRef;
use alloc::{string::String, vec::Vec};
use musicxml_internal::*;
use musicxml_macros::*;

/// Attributes pertaining to the [Play] element.
#[derive(Debug, Default, PartialEq, Eq, AttributeDeserialize, AttributeSerialize)]
pub struct PlayAttributes {
  /// Specifies an ID that is unique to the entire document.
  pub id: Option<IdRef>,
}

/// Contents of the [Play] element.
#[derive(Debug, Default, PartialEq, Eq, ContentDeserialize, ContentSerialize)]
pub struct PlayContents {
  /// The [Ipa] element specifies International Phonetic Alphabet symbols for vocal music.
  pub ipa: Option<Ipa>,
  /// The [Mute] element specifies muting techniques to be used in conjunction with the [InstrumentSound][super::InstrumentSound] element.
  pub mute: Option<Mute>,
  /// The [SemiPitched] element specifies pitch modifications to be used in conjunction with the [InstrumentSound][super::InstrumentSound] element.
  pub semi_pitched: Option<SemiPitched>,
  /// The [OtherPlay] element specifies other playback techniques to be used in conjunction with the [InstrumentSound][super::InstrumentSound] element.
  pub other_play: Option<OtherPlay>,
}

/// The [Play] element specifies playback techniques to be used in conjunction with the [InstrumentSound][super::InstrumentSound] element.
///
/// When used as part of a [Sound][super::Sound] element, it applies to all notes going forward in score order. In multi-instrument parts,
/// the affected instrument should be specified using the id attribute. When used as part of a [Note][super::Note] element, it applies to the current note only.
#[derive(Debug, PartialEq, Eq, ElementDeserialize, ElementSerialize)]
pub struct Play {
  /// Element-specific attributes
  pub attributes: PlayAttributes,
  #[flatten]
  /// Element-specific content
  pub content: PlayContents,
}
