use alloc::string::String;
use musicxml_internal::{DatatypeDeserializer, DatatypeSerializer};
use musicxml_macros::{DatatypeDeserialize, DatatypeSerialize};

/// Used for the direction of arrows and other pointed symbols like vertical accents, indicating which way the tip is pointing.
#[derive(Debug, PartialEq, Eq, DatatypeDeserialize, DatatypeSerialize)]
pub enum UpDown {
  /// <span class="smufl">&#xE4AC;</span>
  Up,
  /// <span class="smufl">&#xE4AD;</span>
  Down,
}
