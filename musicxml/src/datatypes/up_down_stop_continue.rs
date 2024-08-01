use alloc::string::String;
use musicxml_internal::{DatatypeDeserializer, DatatypeSerializer};
use musicxml_macros::{DatatypeDeserialize, DatatypeSerialize};

/// Used for octave-shift elements, indicating the direction of the shift from their true pitched values because of printing difficulty.
#[derive(Debug, PartialEq, Eq, DatatypeDeserialize, DatatypeSerialize)]
pub enum UpDownStopContinue {
  /// Start of an [OctaveShift][crate::elements::OctaveShift] up, such as 8va bassa.
  Up,
  /// Start of an [OctaveShift][crate::elements::OctaveShift] down, such as 8va.
  Down,
  /// Stop of an [OctaveShift][crate::elements::OctaveShift].
  Stop,
  /// Continuation of an [OctaveShift][crate::elements::OctaveShift], including system breaks.
  Continue,
}
