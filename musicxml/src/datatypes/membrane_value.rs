use musicxml_internal::{DatatypeDeserializer, DatatypeSerializer};
use musicxml_macros::{DatatypeDeserialize, DatatypeSerialize};

/// Represents pictograms for membrane percussion instruments.
#[derive(Debug, PartialEq, Eq, DatatypeDeserialize, DatatypeSerialize)]
pub enum MembraneValue {
  /// <span class="smufl">&#xE6D4;</span>
  #[rename("bass drum")]
  BassDrum,
  /// <span class="smufl">&#xE6D5;</span>
  #[rename("bass drum on side")]
  BassDrumOnSide,
  /// <span class="smufl">&#xE6DD;</span>
  Bongos,
  /// <span class="smufl">&#xE6D8;</span>
  #[rename("Chinese tomtom")]
  ChineseTomtom,
  /// <span class="smufl">&#xE6DE;</span>
  #[rename("conga drum")]
  CongaDrum,
  /// <span class="smufl">&#xE6E4;</span>
  Cuica,
  /// <span class="smufl">&#xE6E2;</span>
  #[rename("goblet drum")]
  GobletDrum,
  /// <span class="smufl">&#xE6DA;</span>
  #[rename("Indo-American tomtom")]
  IndoAmericanTomtom,
  /// <span class="smufl">&#xE6D9;</span>
  #[rename("Japanese tomtom")]
  JapaneseTomtom,
  /// <span class="smufl">&#xE6D3;</span>
  #[rename("military drum")]
  MilitaryDrum,
  /// <span class="smufl">&#xE6D1;</span>
  #[rename("snare drum")]
  SnareDrum,
  /// <span class="smufl">&#xE6D2;</span>
  #[rename("snare drum snares off")]
  SnareDrumSnaresOff,
  /// <span class="smufl">&#xE6E3;</span>
  Tabla,
  /// <span class="smufl">&#xE6DB;</span>
  Tambourine,
  /// <span class="smufl">&#xE6D6;</span>
  #[rename("tenor drum")]
  TenorDrum,
  /// <span class="smufl">&#xE6DC;</span>
  Timbales,
  /// <span class="smufl">&#xE6D7;</span>
  Tomtom,
}
