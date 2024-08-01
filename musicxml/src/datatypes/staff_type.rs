use alloc::string::String;
use musicxml_internal::{DatatypeDeserializer, DatatypeSerializer};
use musicxml_macros::{DatatypeDeserialize, DatatypeSerialize};

/// Specifies different uses for the staff.
#[derive(Debug, PartialEq, Eq, DatatypeDeserialize, DatatypeSerialize)]
pub enum StaffType {
  /// Shares the same music as the prior staff, but displayed differently
  /// (e.g., treble and bass clef, standard notation and tablature). It is not included in playback.
  /// An alternate staff provides more information to an application reading a file than encoding
  /// the same music in separate parts, so its use is preferred in this situation if feasible.
  Alternate,
  /// Represents music from another part.
  Cue,
  /// Represents musical alternatives created by an editor rather than the composer.
  /// It can be used for suggested interpretations or alternatives from other sources.
  Editorial,
  /// Represents music that can be played instead of what appears on the regular staff.
  Ossia,
  /// Represents the standard default staff-type.
  Regular,
}
