use crate::datatypes::Token;
use musicxml_internal::*;
use musicxml_macros::*;

/// Attributes pertaining to the [Rights] element.
#[derive(Debug, PartialEq, Eq, AttributeDeserialize, AttributeSerialize)]
pub struct RightsAttributes {
  /// Standard type values are music, words, and arrangement, but other types may be used.
  /// This attribute is only needed when there are multiple [Rights] elements.
  pub r#type: Token,
}

/// The [Rights] element contains copyright and other intellectual property notices.
/// 
/// This is similar to the [Dublin Core rights](https://www.dublincore.org/specifications/dublin-core/dcmi-terms/elements11/rights/) element.
/// Words, music, and derivatives can have different types, so multiple [Rights] elements with different type attributes are supported.
#[derive(Debug, PartialEq, Eq, ElementDeserialize, ElementSerialize)]
pub struct Rights {
  /// Element-specific attributes
  pub attributes: RightsAttributes,
  /// Element-specific content
  pub content: String,
}
