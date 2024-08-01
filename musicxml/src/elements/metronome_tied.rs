use crate::datatypes::StartStop;
use alloc::{string::String, vec::Vec};
use musicxml_internal::*;
use musicxml_macros::*;

/// Attributes pertaining to the [MetronomeTied] element.
#[derive(Debug, PartialEq, Eq, AttributeDeserialize, AttributeSerialize)]
pub struct MetronomeTiedAttributes {
  /// Indicates if this is the start or stop of the tie.
  pub r#type: StartStop,
}

/// The [MetronomeTied] element indicates the presence of a tie within a metric relationship mark.
///
/// As with the [Tied][super::Tied] element, both the start and stop of the tie should be specified, in this case within separate
/// [MetronomeNote][super::MetronomeNote] elements.
#[derive(Debug, PartialEq, Eq, ElementDeserialize, ElementSerialize)]
#[rename("metronome-tied")]
pub struct MetronomeTied {
  /// Element-specific attributes
  pub attributes: MetronomeTiedAttributes,
  /// Element-specific content
  pub content: (),
}
