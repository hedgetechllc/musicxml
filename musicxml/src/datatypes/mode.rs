use musicxml_internal::{DatatypeDeserializer, DatatypeSerializer};
use musicxml_macros::{DatatypeDeserialize, DatatypeSerialize};

/// Used to specify major/minor and other mode distinctions.
#[derive(Debug, PartialEq, Eq, DatatypeDeserialize, DatatypeSerialize)]
pub enum Mode {
  /// Major mode.
  Major,
  /// Minor mode.
  Minor,
  /// Dorian mode.
  Dorian,
  /// Phrygian mode.
  Phrygian,
  /// Lydian mode.
  Lydian,
  /// Mixolydian mode.
  Mixolydian,
  /// Aeolian mode.
  Aeolian,
  /// Locrian mode.
  Locrian,
  /// Ionian mode.
  Ionian,
  /// No mode.
  None,
}
