use crate::datatypes::YesNo;
use alloc::{string::String, vec::Vec};
use musicxml_internal::*;
use musicxml_macros::*;

/// Attributes pertaining to the [Double] element.
#[derive(Debug, Default, PartialEq, Eq, AttributeDeserialize, AttributeSerialize)]
pub struct DoubleAttributes {
  /// If the `above` attribute is set to yes, the doubling is one octave above what is written,
  /// as for mixed flute / piccolo parts in band literature. Otherwise the doubling is one octave
  /// below what is written, as for mixed cello / bass parts in orchestral literature.
  pub above: Option<YesNo>,
}

/// If the [Double] element is present, it indicates that the music is doubled one octave from what is currently written.
#[derive(Debug, PartialEq, Eq, ElementDeserialize, ElementSerialize)]
pub struct Double {
  /// Element-specific attributes
  pub attributes: DoubleAttributes,
  /// Element-specific content
  pub content: (),
}
