use alloc::string::String;
use musicxml_internal::{DatatypeDeserializer, DatatypeSerializer};
use musicxml_macros::{DatatypeDeserialize, DatatypeSerialize};

/// Represents pictograms for wood percussion instruments.
///
/// The [Maraca][WoodValue::Maraca] and [Maracas][WoodValue::Maracas] values distinguish
/// the one- and two-maraca versions of the pictogram.
#[derive(Debug, PartialEq, Eq, DatatypeDeserialize, DatatypeSerialize)]
pub enum WoodValue {
  /// <span class="smufl">&#xE6FB;</span>
  #[rename("bamboo scraper")]
  BambooScraper,
  /// <span class="smufl">&#xE6F7;</span>
  #[rename("board clapper")]
  BoardClapper,
  /// <span class="smufl">&#xE743;</span>
  Cabasa,
  /// <span class="smufl">&#xE6F8;</span>
  Castanets,
  /// <span class="smufl">&#xE6F9;</span>
  #[rename("castanets with handle")]
  CastanetsWithHandle,
  /// <span class="smufl">&#xE6F2;</span>
  Claves,
  /// <span class="smufl">&#xE6F5;</span>
  #[rename("football rattle")]
  FootballRattle,
  /// <span class="smufl">&#xE6F3;</span>
  Guiro,
  /// <span class="smufl">&#xE6DF;</span>
  #[rename("log drum")]
  LogDrum,
  /// <span class="smufl">&#xE741;</span>
  Maraca,
  /// <span class="smufl">&#xE742;</span>
  Maracas,
  /// <span class="smufl">&#xE6FA;</span>
  Quijada,
  /// <span class="smufl">&#xE747;</span>
  Rainstick,
  /// <span class="smufl">&#xE6F4;</span>
  Ratchet,
  /// <span class="smufl">&#xE6FC;</span>
  #[rename("reco-reco")]
  RecoReco,
  /// <span class="smufl">&#xE762;</span>
  #[rename("sandpaper blocks")]
  SandpaperBlocks,
  /// <span class="smufl">&#xE6E0;</span>
  #[rename("slit drum")]
  SlitDrum,
  /// <span class="smufl">&#xE6F1;</span>
  #[rename("temple block")]
  TempleBlock,
  /// <span class="smufl">&#xE745;</span>
  Vibraslap,
  /// <span class="smufl">&#xE6F6;</span>
  Whip,
  /// <span class="smufl">&#xE6F0;</span>
  #[rename("wood block")]
  WoodBlock,
}
