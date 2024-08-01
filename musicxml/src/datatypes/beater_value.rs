use alloc::string::String;
use musicxml_internal::{DatatypeDeserializer, DatatypeSerializer};
use musicxml_macros::{DatatypeDeserialize, DatatypeSerialize};

/// Represents pictograms for beaters, mallets, and sticks that do not have different materials represented in the pictogram.
#[derive(Debug, PartialEq, Eq, DatatypeDeserialize, DatatypeSerialize)]
pub enum BeaterValue {
  /// <span class="smufl">&#xE7DE;</span>
  Bow,
  /// <span class="smufl">&#xE7DF;</span>
  #[rename("chime hammer")]
  ChimeHammer,
  /// <span class="smufl">&#xE7E7;</span>
  Coin,
  /// <span class="smufl">&#xE7E8;</span>
  #[rename("drum stick")]
  DrumStick,
  /// <span class="smufl">&#xE7E4;</span>
  Finger,
  /// <span class="smufl">&#xE7E6;</span>
  Fingernail,
  /// <span class="smufl">&#xE7E5;</span>
  Fist,
  /// <span class="smufl">&#xE7DD;</span>
  #[rename("guiro scraper")]
  GuiroScraper,
  /// <span class="smufl">&#xE7E1;</span>
  Hammer,
  /// <span class="smufl">&#xE7E3;</span>
  Hand,
  /// <span class="smufl">&#xE7D3;</span>
  #[rename("jazz stick")]
  JazzStick,
  /// <span class="smufl">&#xE7E2;</span>
  #[rename("knitting needle")]
  KnittingNeedle,
  /// <span class="smufl">&#xE7E0;</span>
  #[rename("metal hammer")]
  MetalHammer,
  /// <span class="smufl">&#xE734;</span>
  #[rename("slide brush on gong")]
  SlideBrushOnGong,
  /// <span class="smufl">&#xE7D1;</span>
  #[rename("snare stick")]
  SnareStick,
  /// <span class="smufl">&#xE7DC;</span>
  #[rename("spoon mallet")]
  SpoonMallet,
  /// <span class="smufl">&#xE7B2;</span>
  Superball,
  /// <span class="smufl">&#xE7D5;</span>
  #[rename("triangle beater")]
  TriangleBeater,
  /// <span class="smufl">&#xE7EF;</span>
  #[rename("triangle beater plain")]
  TriangleBeaterPlain,
  /// <span class="smufl">&#xE7D7;</span>
  #[rename("wire brush")]
  WireBrush,
}
