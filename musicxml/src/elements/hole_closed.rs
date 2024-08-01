use crate::datatypes::{HoleClosedLocation, HoleClosedValue};
use alloc::{string::String, vec::Vec};
use musicxml_internal::*;
use musicxml_macros::*;

/// Attributes pertaining to the [HoleClosed] element.
#[derive(Debug, Default, PartialEq, Eq, AttributeDeserialize, AttributeSerialize)]
pub struct HoleClosedAttributes {
  /// Indicates which portion of the hole is filled in when the element value is half.
  pub location: Option<HoleClosedLocation>,
}

/// The [HoleClosed] element represents whether the hole is closed, open, or half-open.
#[derive(Debug, PartialEq, Eq, ElementDeserialize, ElementSerialize)]
#[rename("hole-closed")]
pub struct HoleClosed {
  /// Element-specific attributes
  pub attributes: HoleClosedAttributes,
  /// Element-specific content
  pub content: HoleClosedValue,
}
