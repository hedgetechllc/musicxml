use crate::datatypes::{DistanceType, Tenths};
use alloc::{string::String, vec::Vec};
use musicxml_internal::*;
use musicxml_macros::*;

/// Attributes pertaining to the [Distance] element.
#[derive(Debug, PartialEq, Eq, AttributeDeserialize, AttributeSerialize)]
pub struct DistanceAttributes {
  /// The type of distance being defined.
  pub r#type: DistanceType,
}

/// The [Distance] element represents standard distances between notation elements in tenths.
#[derive(Debug, PartialEq, Eq, ElementDeserialize, ElementSerialize)]
pub struct Distance {
  /// Element-specific attributes
  pub attributes: DistanceAttributes,
  /// Element-specific content
  pub content: Tenths,
}
