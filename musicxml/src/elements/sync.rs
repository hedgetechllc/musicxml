use crate::datatypes::{IdRef, Milliseconds, SyncType, TimeOnly};
use alloc::{string::String, vec::Vec};
use musicxml_internal::*;
use musicxml_macros::*;

/// Attributes pertaining to the [Sync] element.
#[derive(Debug, PartialEq, Eq, AttributeDeserialize, AttributeSerialize)]
pub struct SyncAttributes {
  /// Specifies the style that a score following application should use to synchronize an accompaniment with a performer.
  pub r#type: SyncType,
  /// Specifies a latency time in milliseconds that the listening application should expect from the performer.
  pub latency: Option<Milliseconds>,
  /// Restricts the element to apply to a single [Player][super::Player].
  pub player: Option<IdRef>,
  /// Restricts the type to apply to a set of times through a repeated section.
  /// If missing, the type applies all times through the repeated section.
  pub time_only: Option<TimeOnly>,
}

/// The [Sync] element specifies the style that a score-following application should use the synchronize an accompaniment with a performer.
///
/// If this element is not included in a score, default synchronization depends on the application.
#[derive(Debug, PartialEq, Eq, ElementDeserialize, ElementSerialize)]
pub struct Sync {
  /// Element-specific attributes
  pub attributes: SyncAttributes,
  /// Element-specific content
  pub content: (),
}
