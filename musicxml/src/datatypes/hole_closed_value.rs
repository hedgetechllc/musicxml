use musicxml_internal::{DatatypeDeserializer, DatatypeSerializer};
use musicxml_macros::{DatatypeDeserialize, DatatypeSerialize};

/// Represents whether the hole is closed, open, or half-open.
#[derive(Debug, PartialEq, Eq, DatatypeDeserialize, DatatypeSerialize)]
pub enum HoleClosedValue {
  /// <span class="smufl">&#xE5F4;</span>
  Yes,
  /// <span class="smufl">&#xE5F9;</span>
  No,
  /// <span class="smufl">&#xE5F6;</span>
  Half,
}
