use alloc::string::String;
use musicxml_internal::{DatatypeDeserializer, DatatypeSerializer};
use musicxml_macros::{DatatypeDeserialize, DatatypeSerialize};

/// Represents the type of handbell technique being notated.
#[derive(Debug, PartialEq, Eq, DatatypeDeserialize, DatatypeSerialize)]
pub enum HandbellValue {
  /// <span class="smufl">&#xE81F;</span>
  Belltree,
  /// <span class="smufl">&#xE81E;</span>
  Damp,
  /// <span class="smufl">&#xE81B;</span>
  Echo,
  /// <span class="smufl">&#xE81D;</span>
  Gyro,
  /// <span class="smufl">&#xE812;</span>
  #[rename("hand martellato")]
  HandMartellato,
  /// <span class="smufl">&#xE816;</span>
  #[rename("mallet lift")]
  MalletLift,
  /// <span class="smufl">&#xE815;</span>
  #[rename("mallet table")]
  MalletTable,
  /// <span class="smufl">&#xE810;</span>
  Martellato,
  /// <span class="smufl">&#xE811;</span>
  #[rename("martellato lift")]
  MartellatoLift,
  /// <span class="smufl">&#xE813;</span>
  #[rename("muted martellato")]
  MutedMartellato,
  /// <span class="smufl">&#xE817;</span>
  #[rename("pluck lift")]
  PluckLift,
  /// <span class="smufl">&#xE81A;</span>
  Swing,
}
