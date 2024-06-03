use musicxml_internal::{DatatypeDeserializer, DatatypeSerializer};
use musicxml_macros::{DatatypeDeserialize, DatatypeSerialize};

/// Represents notated accidentals supported by MusicXML.
///
/// The Quarter- and ThreeQuarters- accidentals are Tartini-style quarter-tone accidentals.
/// The -Down and -Up accidentals are quarter-tone accidentals that include arrows pointing down or up.
/// The Slash- accidentals are used in Turkish classical music.
/// The numbered Sharp and Flat accidentals are superscripted versions of the accidental signs, used in Turkish folk music.
/// The Sori and Koron accidentals are microtonal sharp and flat accidentals used in Iranian and Persian music.
/// The Other accidental covers accidentals other than those listed here. It is usually used in combination with a
/// `smufl` attribute to specify a particular Standard Music Font Layout (SMuFL) accidental.
///
/// The `smufl` attribute may be used with any accidental value to help specify the appearance of symbols that share the same MusicXML semantics.
#[derive(Debug, PartialEq, Eq, DatatypeDeserialize, DatatypeSerialize)]
pub enum AccidentalValue {
  /// <span class="smufl">&#xE262;</span>
  Sharp,
  /// <span class="smufl">&#xE261;</span>
  Natural,
  /// <span class="smufl">&#xE260;</span>
  Flat,
  /// <span class="smufl">&#xE263;</span>
  #[rename("double-sharp")]
  DoubleSharp,
  /// <span class="smufl">&#xE269;</span>
  #[rename("sharp-sharp")]
  SharpSharp,
  /// <span class="smufl">&#xE264;</span>
  #[rename("flat-flat")]
  FlatFlat,
  /// <span class="smufl">&#xE268;</span>
  #[rename("natural-sharp")]
  NaturalSharp,
  /// <span class="smufl">&#xE267;</span>
  #[rename("natural-flat")]
  NaturalFlat,
  /// <span class="smufl">&#xE280;</span>
  #[rename("quarter-flat")]
  QuarterFlat,
  /// <span class="smufl">&#xE282;</span>
  #[rename("quarter-sharp")]
  QuarterSharp,
  /// <span class="smufl">&#xE281;</span>
  #[rename("three-quarters-flat")]
  ThreeQuartersFlat,
  /// <span class="smufl">&#xE283;</span>
  #[rename("three-quarters-sharp")]
  ThreeQuartersSharp,
  /// <span class="smufl">&#xE275;</span>
  #[rename("sharp-down")]
  SharpDown,
  /// <span class="smufl">&#xE274;</span>
  #[rename("sharp-up")]
  SharpUp,
  /// <span class="smufl">&#xE273;</span>
  #[rename("natural-down")]
  NaturalDown,
  /// <span class="smufl">&#xE272;</span>
  #[rename("natural-up")]
  NaturalUp,
  /// <span class="smufl">&#xE271;</span>
  #[rename("flat-down")]
  FlatDown,
  /// <span class="smufl">&#xE270;</span>
  #[rename("flat-up")]
  FlatUp,
  /// <span class="smufl">&#xE277;</span>
  #[rename("double-sharp-down")]
  DoubleSharpDown,
  /// <span class="smufl">&#xE276;</span>
  #[rename("double-sharp-up")]
  DoubleSharpUp,
  /// <span class="smufl">&#xE279;</span>
  #[rename("flat-flat-down")]
  FlatFlatDown,
  /// <span class="smufl">&#xE278;</span>
  #[rename("flat-flat-up")]
  FlatFlatUp,
  /// <span class="smufl">&#xE27B;</span>
  #[rename("arrow-down")]
  ArrowDown,
  /// <span class="smufl">&#xE27A;</span>
  #[rename("arrow-up")]
  ArrowUp,
  /// <span class="smufl">&#xE265;</span>
  #[rename("triple-sharp")]
  TripleSharp,
  /// <span class="smufl">&#xE266;</span>
  #[rename("triple-flat")]
  TripleFlat,
  /// <span class="smufl">&#xE446;</span>
  #[rename("slash-quarter-sharp")]
  SlashQuarterSharp,
  /// <span class="smufl">&#xE447;</span>
  #[rename("slash-sharp")]
  SlashSharp,
  /// <span class="smufl">&#xE442;</span>
  #[rename("slash-flat")]
  SlashFlat,
  /// <span class="smufl">&#xE440;</span>
  #[rename("double-slash-flat")]
  DoubleSlashFlat,
  /// <span class="smufl">&#xE450;</span>
  #[rename("sharp-1")]
  Sharp1,
  /// <span class="smufl">&#xE451;</span>
  #[rename("sharp-2")]
  Sharp2,
  /// <span class="smufl">&#xE452;</span>
  #[rename("sharp-3")]
  Sharp3,
  /// <span class="smufl">&#xE453;</span>
  #[rename("sharp-5")]
  Sharp5,
  /// <span class="smufl">&#xE454;</span>
  #[rename("flat-1")]
  Flat1,
  /// <span class="smufl">&#xE455;</span>
  #[rename("flat-2")]
  Flat2,
  /// <span class="smufl">&#xE456;</span>
  #[rename("flat-3")]
  Flat3,
  /// <span class="smufl">&#xE457;</span>
  #[rename("flat-4")]
  Flat4,
  /// <span class="smufl">&#xE461;</span>
  Sori,
  /// <span class="smufl">&#xE460;</span>
  Koron,
  /// Covers accidentals other than those listed here. It is usually used in combination with the `smufl` attribute to specify a particular Standard Music Font Layout (SMuFL) accidental.
  Other,
}

#[cfg(test)]
mod accidental_value_tests {
  use super::*;

  #[test]
  fn serialize_valid1() {
    let test = AccidentalValue::DoubleSharpUp;
    let result = AccidentalValue::serialize(&test);
    assert_eq!(result, "double-sharp-up");
  }

  #[test]
  fn deserialize_valid1() {
    let result = AccidentalValue::deserialize("double-sharp-up");
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), AccidentalValue::DoubleSharpUp);
  }

  #[test]
  fn deserialize_valid2() {
    let result = AccidentalValue::deserialize("sharp-sharp-up");
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), AccidentalValue::DoubleSharpUp);
  }

  #[test]
  fn deserialize_invalid() {
    let result = AccidentalValue::deserialize("sharp-shasrp-up");
    assert!(result.is_err());
  }
}
