use crate::datatypes::{IdRef, TimeOnly, Token};
use alloc::{string::String, vec::Vec};
use musicxml_internal::*;
use musicxml_macros::*;

/// Attributes pertaining to the [OtherListening] element.
#[derive(Debug, PartialEq, Eq, AttributeDeserialize, AttributeSerialize)]
pub struct OtherListeningAttributes {
  /// Indicates the type of listening to which the element content applies.
  pub r#type: Token,
  /// Restricts the element to apply to a single player.
  pub player: Option<IdRef>,
  /// Restricts the type to apply to a set of times through a repeated section.
  /// If missing, the type applies all times through the repeated section.
  pub time_only: Option<TimeOnly>,
}

/// The [OtherListening] element represents other types of listening control and interaction that change the state of the
/// listening application from the specified point in the performance onward.
#[derive(Debug, PartialEq, Eq, ElementDeserialize, ElementSerialize)]
#[rename("other-listening")]
pub struct OtherListening {
  /// Element-specific attributes
  pub attributes: OtherListeningAttributes,
  /// Element-specific content
  pub content: String,
}
