use alloc::string::String;
use musicxml_internal::{DatatypeDeserializer, DatatypeSerializer};
use musicxml_macros::{DatatypeDeserialize, DatatypeSerialize};

/// Represents pictograms for pitched percussion instruments.
///
/// The [Chimes][PitchedValue::Chimes] and [TubularChimes][PitchedValue::TubularChimes] values
/// distinguish the single-line and double-line versions of the pictogram.
#[derive(Debug, PartialEq, Eq, DatatypeDeserialize, DatatypeSerialize)]
pub enum PitchedValue {
  /// <span class="smufl">&#xE6B0;</span>
  Celesta,
  /// <span class="smufl">&#xE6C2;</span>
  Chimes,
  /// <span class="smufl">&#xE6A0;</span>
  Glockenspiel,
  /// <span class="smufl">&#xE6B1;</span>
  Lithophone,
  /// <span class="smufl">&#xE6A9;</span>
  Mallet,
  /// <span class="smufl">&#xE6A6;</span>
  Marimba,
  /// <span class="smufl">&#xE6AF;</span>
  #[rename("steel drums")]
  SteelDrums,
  /// <span class="smufl">&#xE6B2;</span>
  Tubaphone,
  /// <span class="smufl">&#xE6C0;</span>
  #[rename("tubular chimes")]
  TubularChimes,
  /// <span class="smufl">&#xE6A7;</span>
  Vibraphone,
  /// <span class="smufl">&#xE6A1;</span>
  Xylophone,
}
