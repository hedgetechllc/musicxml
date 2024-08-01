use alloc::string::String;
use musicxml_internal::{DatatypeDeserializer, DatatypeSerializer};
use musicxml_macros::{DatatypeDeserialize, DatatypeSerialize};

/// Differentiates different types of harmonies when alternate harmonies are possible.
#[derive(Debug, PartialEq, Eq, DatatypeDeserialize, DatatypeSerialize)]
pub enum HarmonyType {
  /// Alternate analysis.
  Alternate,
  /// All notes present in the music.
  Explicit,
  /// Some notes are missing but implied.
  Implied,
}
