use crate::datatypes::{StartStop, TimeOnly};
use musicxml_internal::*;
use musicxml_macros::*;

/// Attributes pertaining to the [Tie] element.
#[derive(Debug, PartialEq, Eq, AttributeDeserialize, AttributeSerialize)]
pub struct TieAttributes {
  /// Indicates if this is the start or stop of the tie.
  pub r#type: StartStop,
  /// Restricts the type to apply to a set of times through a repeated section.
  /// If missing, the type applies all times through the repeated section.
  pub time_only: Option<TimeOnly>,
}

/// The [Tie] element indicates that a tie begins or ends with this note.
/// 
/// The [Tie] element indicates sound; the [Tied][super::Tied] element indicates notation.
#[derive(Debug, PartialEq, Eq, ElementDeserialize, ElementSerialize)]
pub struct Tie {
  /// Element-specific attributes
  pub attributes: TieAttributes,
  /// Element-specific content
  pub content: (),
}
