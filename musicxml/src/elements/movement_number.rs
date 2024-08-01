use alloc::string::String;
use musicxml_internal::*;
use musicxml_macros::*;

/// The [MovementNumber] element specifies the number of a movement.
#[derive(Debug, PartialEq, Eq, ElementDeserialize, ElementSerialize)]
#[rename("movement-number")]
pub struct MovementNumber {
  /// Element-specific attributes
  pub attributes: (),
  /// Element-specific content
  pub content: String,
}
