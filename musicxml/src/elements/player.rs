use super::PlayerName;
use crate::datatypes::Id;
use musicxml_internal::*;
use musicxml_macros::*;

/// Attributes pertaining to the [Player] element.
#[derive(Debug, PartialEq, Eq, AttributeDeserialize, AttributeSerialize)]
pub struct PlayerAttributes {
  /// Specifies an ID that is unique to the entire document.
  pub id: Id,
}

/// Contents of the [Player] element.
#[derive(Debug, PartialEq, Eq, ContentDeserialize, ContentSerialize)]
pub struct PlayerContents {
  /// The [PlayerName] element specifies the name of the player.
  pub player_name: PlayerName,
}

/// The [Player] element allows for multiple players per [ScorePart][super::ScorePart] for use in listening applications.
///
/// One player may play multiple instruments, while a single instrument may include multiple players in divisi sections.
#[derive(Debug, PartialEq, Eq, ElementDeserialize, ElementSerialize)]
pub struct Player {
  /// Element-specific attributes
  pub attributes: PlayerAttributes,
  #[flatten]
  /// Element-specific content
  pub content: PlayerContents,
}
