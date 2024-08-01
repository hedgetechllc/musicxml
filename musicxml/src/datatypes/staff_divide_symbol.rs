use alloc::string::String;
use musicxml_internal::{DatatypeDeserializer, DatatypeSerializer};
use musicxml_macros::{DatatypeDeserialize, DatatypeSerialize};

/// Used for staff division symbols.
///
/// The down, up, and up-down values correspond to SMuFL code points U+E00B, U+E00C, and U+E00D respectively.
#[derive(Debug, Default, PartialEq, Eq, DatatypeDeserialize, DatatypeSerialize)]
pub enum StaffDivideSymbol {
  /// <span class="smufl">&#xE00B;</span>
  Down,
  /// <span class="smufl">&#xE00C;</span>
  Up,
  /// <span class="smufl">&#xE00D;</span>
  #[default]
  #[rename("up-down")]
  UpDown,
}
