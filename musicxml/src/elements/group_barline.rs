use crate::datatypes::{Color, GroupBarlineValue};
use alloc::{string::String, vec::Vec};
use musicxml_internal::*;
use musicxml_macros::*;

/// Attributes pertaining to the [GroupBarline] element.
#[derive(Debug, Default, PartialEq, Eq, AttributeDeserialize, AttributeSerialize)]
pub struct GroupBarlineAttributes {
  /// Indicates the color of an element.
  pub color: Option<Color>,
}

/// The [GroupBarline] element indicates if the group should have common barlines.
#[derive(Debug, PartialEq, Eq, ElementDeserialize, ElementSerialize)]
#[rename("group-barline")]
pub struct GroupBarline {
  /// Element-specific attributes
  pub attributes: GroupBarlineAttributes,
  /// Element-specific content
  pub content: GroupBarlineValue,
}
