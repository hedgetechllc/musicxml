use alloc::string::String;
use musicxml_internal::{DatatypeDeserializer, DatatypeSerializer};
use musicxml_macros::{DatatypeDeserialize, DatatypeSerialize};

/// Indicates which symbol should be used in specifying a degree.
#[derive(Debug, PartialEq, Eq, DatatypeDeserialize, DatatypeSerialize)]
pub enum DegreeSymbolValue {
  /// <span class="smufl">&#xE873;</span>
  Major,
  /// <span class="smufl">&#xE874;</span>
  Minor,
  /// <span class="smufl">&#xE872;</span>
  Augmented,
  /// <span class="smufl">&#xE870;</span>
  Diminished,
  /// <span class="smufl">&#xE871;</span>
  #[rename("half-diminished")]
  HalfDiminished,
}
