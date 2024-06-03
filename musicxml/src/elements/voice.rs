use musicxml_internal::*;
use musicxml_macros::*;

/// A [Voice] is a sequence of musical events (e.g. notes, chords, rests) that proceeds linearly in time.
///
/// ![Voice](voice.png)
///
/// The [Voice] element is used to distinguish between multiple voices in individual parts.
#[derive(Debug, PartialEq, Eq, ElementDeserialize, ElementSerialize)]
pub struct Voice {
  /// Element-specific attributes
  pub attributes: (),
  /// Element-specific content
  pub content: String,
}
