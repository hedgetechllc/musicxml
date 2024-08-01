use crate::datatypes::Token;
use alloc::{string::String, vec::Vec};
use musicxml_internal::*;
use musicxml_macros::*;

/// Attributes pertaining to the [Relation] element.
#[derive(Debug, PartialEq, Eq, AttributeDeserialize, AttributeSerialize)]
pub struct RelationAttributes {
  /// Standard type values are music, words, and arrangement, but other types may be used.
  pub r#type: Token,
}

/// The [Relation] element describes a related resource for the music that is encoded.
///
/// This is similar to the [Dublin Core relation](https://www.dublincore.org/specifications/dublin-core/dcmi-terms/elements11/relation/) element.
#[derive(Debug, PartialEq, Eq, ElementDeserialize, ElementSerialize)]
pub struct Relation {
  /// Element-specific attributes
  pub attributes: RelationAttributes,
  /// Element-specific content
  pub content: String,
}
