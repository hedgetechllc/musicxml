use alloc::string::String;
use musicxml_internal::{DatatypeDeserializer, DatatypeSerializer};
use musicxml_macros::{DatatypeDeserialize, DatatypeSerialize};

/// Used to specify [Ending][crate::elements::Ending] types.
#[derive(Debug, PartialEq, Eq, DatatypeDeserialize, DatatypeSerialize)]
pub enum StartStopDiscontinue {
  /// Used with the left barline of the first measure in an ending.
  Start,
  /// Used with the right barline of the last measure in an ending.
  ///
  /// Indicates the ending mark concludes with a downward jog, as is typical for first endings.
  Stop,
  /// Used with the right barline of the last measure in an ending.
  ///
  /// Indicates there is no downward jog, as is typical for second endings that do not conclude a piece.
  Discontinue,
}
