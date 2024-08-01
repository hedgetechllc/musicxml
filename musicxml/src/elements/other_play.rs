use crate::datatypes::Token;
use alloc::{string::String, vec::Vec};
use musicxml_internal::*;
use musicxml_macros::*;

/// Attributes pertaining to the [OtherPlay] element.
#[derive(Debug, PartialEq, Eq, AttributeDeserialize, AttributeSerialize)]
pub struct OtherPlayAttributes {
  /// Indicates the type of playback to which the element content applies.
  pub r#type: Token,
}

/// The [OtherPlay] element represents other types of playback not otherwise specified within the [Play][super::Play] element.
#[derive(Debug, PartialEq, Eq, ElementDeserialize, ElementSerialize)]
#[rename("other-play")]
pub struct OtherPlay {
  /// Element-specific attributes
  pub attributes: OtherPlayAttributes,
  /// Element-specific content
  pub content: String,
}
