use crate::datatypes::Percent;
use musicxml_internal::*;
use musicxml_macros::*;

/// The [Volume] element value is a percentage of the maximum ranging from 0 to 100, with decimal values allowed.
/// 
/// This corresponds to a scaling value for the MIDI 1.0 channel volume controller.
#[derive(Debug, PartialEq, Eq, ElementDeserialize, ElementSerialize)]
pub struct Volume {
  /// Element-specific attributes
  pub attributes: (),
  /// Element-specific content
  pub content: Percent,
}
