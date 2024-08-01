use alloc::string::String;
use musicxml_internal::{DatatypeDeserializer, DatatypeSerializer};
use musicxml_macros::{DatatypeDeserialize, DatatypeSerialize};

/// Represents whether the harmon mute is closed, open, or half-open.
#[derive(Debug, PartialEq, Eq, DatatypeDeserialize, DatatypeSerialize)]
pub enum HarmonClosedValue {
  /// <span class="smufl">&#xE5E8;</span>
  Yes,
  /// <span class="smufl">&#xE5EB;</span>
  No,
  /// <span class="smufl">&#xE5E9;</span>
  Half,
}
