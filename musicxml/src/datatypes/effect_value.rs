use musicxml_internal::{DatatypeDeserializer, DatatypeSerializer};
use musicxml_macros::{DatatypeDeserialize, DatatypeSerialize};

/// Represents pictograms for sound effect percussion instruments.
///
/// The cannon, lotus flute, and megaphone values are in addition to Stone's list.
#[derive(Debug, PartialEq, Eq, DatatypeDeserialize, DatatypeSerialize)]
pub enum EffectValue {
  /// <span class="smufl">&#xE701;</span>
  Anvil,
  /// <span class="smufl">&#xE755;</span>
  #[rename("auto horn")]
  AutoHorn,
  /// <span class="smufl">&#xE751;</span>
  #[rename("bird whistle")]
  BirdWhistle,
  /// <span class="smufl">&#xE761;</span>
  Cannon,
  /// <span class="smufl">&#xE757;</span>
  #[rename("duck call")]
  DuckCall,
  /// <span class="smufl">&#xE760;</span>
  #[rename("gun shot")]
  GunShot,
  /// <span class="smufl">&#xE756;</span>
  #[rename("klaxon horn")]
  KlaxonHorn,
  /// <span class="smufl">&#xE763;</span>
  #[rename("lions roar")]
  LionsRoar,
  /// <span class="smufl">&#xE75A;</span>
  #[rename("lotus flute")]
  LotusFlute,
  /// <span class="smufl">&#xE759;</span>
  Megaphone,
  /// <span class="smufl">&#xE752;</span>
  #[rename("police whistle")]
  PoliceWhistle,
  /// <span class="smufl">&#xE753;</span>
  Siren,
  /// <span class="smufl">&#xE750;</span>
  #[rename("slide whistle")]
  SlideWhistle,
  /// <span class="smufl">&#xE744;</span>
  #[rename("thunder sheet")]
  ThunderSheet,
  /// <span class="smufl">&#xE754;</span>
  #[rename("wind machine")]
  WindMachine,
  /// <span class="smufl">&#xE758;</span>
  #[rename("wind whistle")]
  WindWhistle,
}
