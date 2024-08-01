use alloc::string::String;
use musicxml_internal::{DatatypeDeserializer, DatatypeSerializer};
use musicxml_macros::{DatatypeDeserialize, DatatypeSerialize};

/// Represents the symbol used for a breath mark.
#[derive(Debug, PartialEq, Eq, DatatypeDeserialize, DatatypeSerialize)]
pub enum BreathMarkValue {
  /// <span class="smufl">&#xE4CE;</span>
  Comma,
  /// <span class="smufl">&#xE4CF;</span>
  Tick,
  /// <span class="smufl">&#xE4D0;</span>
  Upbow,
  /// <span class="smufl">&#xE4D5;</span>
  Salzedo,
  /// The exact symbol used is application-dependent.
  Other,
}
