use musicxml_internal::{DatatypeDeserializer, DatatypeSerializer};
use musicxml_macros::{DatatypeDeserialize, DatatypeSerialize};

/// Represents muting playback for different instruments, including brass, winds, and strings.
/// 
/// The [On][Mute::On] and [Off][Mute::Off] values are used for undifferentiated mutes.
/// The remaining values represent specific mutes.
#[derive(Debug, PartialEq, Eq, DatatypeDeserialize, DatatypeSerialize)]
pub enum Mute {
  /// Represents an on-mute.
  On,
  /// Represents an off-mute.
  Off,
  /// Represents a bucket mute.
  Bucket,
  /// Represents a cup mute.
  Cup,
  /// Represents an echo mute.
  Echo,
  /// Represents a harmon no-stem mute.
  #[rename("harmon-no-stem")]
  HarmonNoStem,
  /// Represents a harmon stem mute.
  #[rename("harmon-stem")]
  HarmonStem,
  /// Represents a hat mute.
  Hat,
  /// Represents a palm mute.
  Palm,
  /// Represents a plunger mute.
  Plunger,
  /// Represents a practice mute.
  Practice,
  /// Represents a solotone mute.
  Solotone,
  /// Represents a stop-hand mute.
  #[rename("stop-hand")]
  StopHand,
  /// Represents a stop mute.
  #[rename("stop-mute")]
  StopMute,
  /// Represents a straight mute.
  Straight,
}
