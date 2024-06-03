use musicxml_internal::*;
use musicxml_macros::*;

/// The [PlayerName] element is typically used within a software application, rather than appearing on the printed page of a score.
#[derive(Debug, PartialEq, Eq, ElementDeserialize, ElementSerialize)]
#[rename("player-name")]
pub struct PlayerName {
  /// Element-specific attributes
  pub attributes: (),
  /// Element-specific content
  pub content: String,
}
