use alloc::string::String;
use musicxml_internal::{DatatypeDeserializer, DatatypeSerializer};
use musicxml_macros::{DatatypeDeserialize, DatatypeSerialize};

/// Represents the shape of pictograms where the material in the stick, mallet, or beater is represented in the pictogram.
#[derive(Debug, PartialEq, Eq, DatatypeDeserialize, DatatypeSerialize)]
pub enum StickType {
  /// <span class="smufl">&#xE798;</span>
  #[rename("bass drum")]
  BassDrum,
  /// <span class="smufl">&#xE7A0;</span>
  #[rename("double bass drum")]
  DoubleBassDrum,
  /// <span class="smufl">&#xE780;</span>
  Glockenspiel,
  /// <span class="smufl">&#xE7BB;</span>
  Gum,
  /// <span class="smufl">&#xE7CD;</span>
  Hammer,
  /// <span class="smufl">&#xE7AE;</span>
  Superball,
  /// <span class="smufl">&#xE788;</span>
  Timpani,
  /// <span class="smufl">&#xE7B3;</span>
  Wound,
  /// <span class="smufl">&#xE770;</span>
  Xylophone,
  /// <span class="smufl">&#xE7A2;</span>
  Yarn,
}
