use alloc::string::String;
use musicxml_internal::{DatatypeDeserializer, DatatypeSerializer};
use musicxml_macros::{DatatypeDeserialize, DatatypeSerialize};

/// Represents pictograms for glass percussion instruments.
#[derive(Debug, PartialEq, Eq, DatatypeDeserialize, DatatypeSerialize)]
pub enum GlassValue {
  /// <span class="smufl">&#xE765;</span>
  #[rename("glass harmonica")]
  GlassHarmonica,
  /// <span class="smufl">&#xE764;</span>
  #[rename("glass harp")]
  GlassHarp,
  /// <span class="smufl">&#xE6C1;</span>
  #[rename("wind chimes")]
  WindChimes,
}
