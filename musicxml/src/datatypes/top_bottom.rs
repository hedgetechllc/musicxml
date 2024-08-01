use alloc::string::String;
use musicxml_internal::{DatatypeDeserializer, DatatypeSerializer};
use musicxml_macros::{DatatypeDeserialize, DatatypeSerialize};

/// Used to indicate the top or bottom part of a vertical shape like [NonArpeggiate][crate::elements::NonArpeggiate].
#[derive(Debug, PartialEq, Eq, DatatypeDeserialize, DatatypeSerialize)]
pub enum TopBottom {
  /// Top of the vertical shape.
  Top,
  /// Bottom of the vertical shape.
  Bottom,
}
