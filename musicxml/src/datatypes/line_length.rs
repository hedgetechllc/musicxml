use alloc::string::String;
use musicxml_internal::{DatatypeDeserializer, DatatypeSerializer};
use musicxml_macros::{DatatypeDeserialize, DatatypeSerialize};

/// Distinguishes between different line lengths for [Doit][crate::elements::Doit],
/// [Falloff][crate::elements::Falloff], [Plop][crate::elements::Plop],
/// and [Scoop][crate::elements::Scoop] articulations.
#[derive(Debug, PartialEq, Eq, DatatypeDeserialize, DatatypeSerialize)]
pub enum LineLength {
  /// <span class="smufl">&#xE5DD;</span>
  Short,
  /// <span class="smufl">&#xE5DE;</span>
  Medium,
  /// <span class="smufl">&#xE5DF;</span>
  Long,
}
