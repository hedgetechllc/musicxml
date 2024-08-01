use alloc::string::String;
use musicxml_internal::{DatatypeDeserializer, DatatypeSerializer};
use musicxml_macros::{DatatypeDeserialize, DatatypeSerialize};

/// Distinguishes types of pedal directions.
#[derive(Debug, PartialEq, Eq, DatatypeDeserialize, DatatypeSerialize)]
pub enum PedalType {
  /// Indicates the start of a damper pedal.
  Start,
  /// 	Indicates a pedal lift without a retake.
  Stop,
  /// Indicates the start of a sostenuto pedal.
  Sostenuto,
  /// Indicates a pedal lift and retake indicated with an inverted V marking.
  ///
  /// Used when the `pedal` line attribute is yes.
  Change,
  /// Allows more precise formatting across system breaks and for more complex pedaling lines.
  ///
  /// Used when the `pedal` line attribute is yes.
  Continue,
  /// Indicates the end of a pedal line that does not include the explicit lift represented by the stop type.
  ///
  /// Used when the `pedal` line attribute is yes.
  Discontinue,
  /// Indicates the start of a pedal line that does not include the downstroke represented by the start type.
  ///
  /// It can be used when a line resumes after being discontinued, or to start a pedal line that is preceded
  /// by a text or symbol representation of the pedal. Used when the `pedal` line attribute is yes.
  Resume,
}
