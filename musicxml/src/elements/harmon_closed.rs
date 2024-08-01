use crate::datatypes::{HarmonClosedLocation, HarmonClosedValue};
use alloc::{string::String, vec::Vec};
use musicxml_internal::*;
use musicxml_macros::*;

/// Attributes pertaining to the [HarmonClosed] element.
#[derive(Debug, Default, PartialEq, Eq, AttributeDeserialize, AttributeSerialize)]
pub struct HarmonClosedAttributes {
  /// Indicates which portion of the symbol is filled in when the element value is half.
  pub location: Option<HarmonClosedLocation>,
}

/// The [HarmonClosed] element represents whether the harmon mute is closed, open, or half-open.
#[derive(Debug, PartialEq, Eq, ElementDeserialize, ElementSerialize)]
#[rename("harmon-closed")]
pub struct HarmonClosed {
  /// Element-specific attributes
  pub attributes: HarmonClosedAttributes,
  /// Element-specific content
  pub content: HarmonClosedValue,
}
