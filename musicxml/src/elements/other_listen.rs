use crate::datatypes::{IdRef, TimeOnly, Token};
use alloc::{string::String, vec::Vec};
use musicxml_internal::*;
use musicxml_macros::*;

/// Attributes pertaining to the [OtherListen] element.
#[derive(Debug, PartialEq, Eq, AttributeDeserialize, AttributeSerialize)]
pub struct OtherListenAttributes {
  /// Indicates the type of listening to which the element content applies.
  pub r#type: Token,
  /// Restricts the element to apply to a single player.
  pub player: Option<IdRef>,
  /// Restricts the type to apply to a set of times through a repeated section.
  /// If missing, the type applies all times through the repeated section.
  pub time_only: Option<TimeOnly>,
}

/// The [OtherListen] element represents other types of listening control and interaction that are specific to a note.
#[derive(Debug, PartialEq, Eq, ElementDeserialize, ElementSerialize)]
#[rename("other-listen")]
pub struct OtherListen {
  /// Element-specific attributes
  pub attributes: OtherListenAttributes,
  /// Element-specific content
  pub content: String,
}
