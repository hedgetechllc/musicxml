use musicxml_internal::{DatatypeDeserializer, DatatypeSerializer};
use musicxml_macros::{DatatypeDeserialize, DatatypeSerialize};

/// Represents pictograms for metal percussion instruments.
/// 
/// The hi-hat value refers to a pictogram like high-hat cymbals, but without the long vertical line at the bottom.
#[derive(Debug, PartialEq, Eq, DatatypeDeserialize, DatatypeSerialize)]
pub enum MetalValue {
  /// <span class="smufl">&#xE717;</span>
  Agogo,
  /// <span class="smufl">&#xE712;</span>
  Almglocken,
  /// <span class="smufl">&#xE714;</span>
  Bell,
  /// <span class="smufl">&#xE71A;</span>
  #[rename("bell plate")]
  BellPlate,
  /// <span class="smufl">&#xE710;</span>
  #[rename("bell tree")]
  BellTree,
  /// <span class="smufl">&#xE6E1;</span>
  #[rename("brake drum")]
  BrakeDrum,
  /// <span class="smufl">&#xE716;</span>
  Cencerro,
  /// <span class="smufl">&#xE748;</span>
  #[rename("chain rattle")]
  ChainRattle,
  /// <span class="smufl">&#xE726;</span>
  #[rename("Chinese cymbal")]
  ChineseCymbal,
  /// <span class="smufl">&#xE711;</span>
  Cowbell,
  /// <span class="smufl">&#xE720;</span>
  #[rename("crash cymbals")]
  CrashCymbals,
  /// <span class="smufl">&#xE6AE;</span>
  Crotale,
  /// <span class="smufl">&#xE728;</span>
  #[rename("cymbal tongs")]
  CymbalTongs,
  /// <span class="smufl">&#xE733;</span>
  #[rename("domed gong")]
  DomedGong,
  /// <span class="smufl">&#xE727;</span>
  #[rename("finger cymbals")]
  FingerCymbals,
  /// <span class="smufl">&#xE740;</span>
  Flexatone,
  /// <span class="smufl">&#xE732;</span>
  Gong,
  /// <span class="smufl">&#xE715;</span>
  Handbell,
  /// <span class="smufl">&#xE722;</span>
  #[rename("hi-hat")]
  HiHat,
  /// <span class="smufl">&#xE723;</span>
  #[rename("high-hat cymbals")]
  HighHatCymbals,
  /// <span class="smufl">&#xE767;</span>
  #[rename("jaw harp")]
  JawHarp,
  /// <span class="smufl">&#xE719;</span>
  #[rename("jingle bells")]
  JingleBells,
  /// <span class="smufl">&#xE766;</span>
  #[rename("musical saw")]
  MusicalSaw,
  /// <span class="smufl">&#xE718;</span>
  #[rename("shell bells")]
  ShellBells,
  /// <span class="smufl">&#xE746;</span>
  Sistrum,
  /// <span class="smufl">&#xE724;</span>
  #[rename("sizzle cymbal")]
  SizzleCymbal,
  /// <span class="smufl">&#xE710;</span>
  #[rename("sleigh bells")]
  SleighBells,
  /// <span class="smufl">&#xE721;</span>
  #[rename("suspended cymbal")]
  SuspendedCymbal,
  /// <span class="smufl">&#xE730;</span>
  #[rename("tam tam")]
  TamTam,
  /// <span class="smufl">&#xE731;</span>
  #[rename("tam tam with beater")]
  TamTamWithBeater,
  /// <span class="smufl">&#xE700;</span>
  Triangle,
  /// <span class="smufl">&#xE725;</span>
  #[rename("Vietnamese hat")]
  VietnameseHat,
}
