use alloc::string::String;
use musicxml_internal::{DatatypeDeserializer, DatatypeSerializer};
use musicxml_macros::{DatatypeDeserialize, DatatypeSerialize};

/// Represents a step of the diatonic scale, represented using the English letters A through G.
#[derive(Debug, PartialEq, Eq, DatatypeDeserialize, DatatypeSerialize)]
pub enum Step {
  /// Diatonic scale step A.
  #[rename("A")]
  A,
  /// Diatonic scale step B.
  #[rename("B")]
  B,
  /// Diatonic scale step C.
  #[rename("C")]
  C,
  /// Diatonic scale step D.
  #[rename("D")]
  D,
  /// Diatonic scale step E.
  #[rename("E")]
  E,
  /// Diatonic scale step F.
  #[rename("F")]
  F,
  /// Diatonic scale step G.
  #[rename("G")]
  G,
}
