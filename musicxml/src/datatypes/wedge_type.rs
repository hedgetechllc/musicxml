use alloc::string::String;
use musicxml_internal::{DatatypeDeserializer, DatatypeSerializer};
use musicxml_macros::{DatatypeDeserialize, DatatypeSerialize};

/// Used to specify [Wedge][crate::elements::Wedge] types.
#[derive(Debug, PartialEq, Eq, DatatypeDeserialize, DatatypeSerialize)]
pub enum WedgeType {
  /// Start of a crescendo [Wedge][crate::elements::Wedge] that is closed on the left side.
  Crescendo,
  /// Start of a diminuendo [Wedge][crate::elements::Wedge]that is closed on the right side.
  Diminuendo,
  /// Stop of a [Wedge][crate::elements::Wedge].
  Stop,
  /// Continuation of a [Wedge][crate::elements::Wedge], including system breaks.
  Continue,
}
