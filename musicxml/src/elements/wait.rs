use crate::datatypes::{IdRef, TimeOnly};
use alloc::{string::String, vec::Vec};
use musicxml_internal::*;
use musicxml_macros::*;

/// Attributes pertaining to the [Wait] element.
#[derive(Debug, Default, PartialEq, Eq, AttributeDeserialize, AttributeSerialize)]
pub struct WaitAttributes {
  /// Restricts the [Wait] to apply to a single player.
  pub player: Option<IdRef>,
  /// Restricts the type to apply to a set of times through a repeated section.
  /// If missing, the type applies all times through the repeated section.
  pub time_only: Option<TimeOnly>,
}

/// The [Wait] element specifies a point where the accompaniment should wait for a performer event before continuing.
///
/// This typically happens at the start of new sections or after a held note or indeterminate music. These waiting points cannot always be
/// inferred reliably from the contents of the displayed score.
#[derive(Debug, PartialEq, Eq, ElementDeserialize, ElementSerialize)]
pub struct Wait {
  /// Element-specific attributes
  pub attributes: WaitAttributes,
  /// Element-specific content
  pub content: (),
}
