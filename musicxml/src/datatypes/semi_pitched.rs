use alloc::string::String;
use musicxml_internal::{DatatypeDeserializer, DatatypeSerializer};
use musicxml_macros::{DatatypeDeserialize, DatatypeSerialize};

/// Represents categories of indefinite pitch for percussion instruments.
#[derive(Debug, PartialEq, Eq, DatatypeDeserialize, DatatypeSerialize)]
pub enum SemiPitched {
  /// High indefinite pitch.
  High,
  /// Low indefinite pitch.
  Low,
  /// Medium indefinite pitch.
  Medium,
  /// Medium-high indefinite pitch.
  #[rename("medium-high")]
  MediumHigh,
  /// Medium-low indefinite pitch.
  #[rename("medium-low")]
  MediumLow,
  /// Very low indefinite pitch.
  #[rename("very-low")]
  VeryLow,
}
