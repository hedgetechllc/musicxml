use musicxml_internal::*;
use musicxml_macros::*;

/// The [InstrumentSound] element describes the default timbre of the [ScoreInstrument][super::ScoreInstrument].
/// 
/// This description is independent of a particular virtual or MIDI instrument specification and allows playback to be shared
/// more easily between applications and libraries.
#[derive(Debug, PartialEq, Eq, ElementDeserialize, ElementSerialize)]
#[rename("instrument-sound")]
pub struct InstrumentSound {
  /// Element-specific attributes
  pub attributes: (),
  /// Element-specific content
  pub content: String,
}
