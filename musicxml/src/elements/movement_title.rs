use musicxml_internal::*;
use musicxml_macros::*;

/// The [MovementTitle] element specifies the title of a movement, not including its number.
#[derive(Debug, PartialEq, Eq, ElementDeserialize, ElementSerialize)]
#[rename("movement-title")]
pub struct MovementTitle {
  /// Element-specific attributes
  pub attributes: (),
  /// Element-specific content
  pub content: String,
}
