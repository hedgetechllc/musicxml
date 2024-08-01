use alloc::string::String;
use musicxml_internal::{DatatypeDeserializer, DatatypeSerializer};
use musicxml_macros::{DatatypeDeserialize, DatatypeSerialize};

/// Represents the symbol to use for a tap element.
///
/// The left and right values refer to the SMuFL guitarLeftHandTapping and guitarRightHandTapping glyphs respectively.
#[derive(Debug, PartialEq, Eq, DatatypeDeserialize, DatatypeSerialize)]
pub enum TapHand {
  /// <span class="smufl">&#xE840;</span>
  Left,
  /// <span class="smufl">&#xE841;</span>
  Right,
}
