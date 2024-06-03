use crate::datatypes::Token;
use musicxml_internal::*;
use musicxml_macros::*;

/// Attributes pertaining to the [Feature] element.
#[derive(Debug, Default, PartialEq, Eq, AttributeDeserialize, AttributeSerialize)]
pub struct FeatureAttributes {
  /// Represents the type of the feature. This type is flexible to allow for different analyses.
  pub r#type: Option<Token>,
}

/// The [Feature] element is a part of the [Grouping][super::Grouping] element used for musical analysis.
/// 
/// The `type` attribute represents the type of the feature and the element content represents its value.
#[derive(Debug, PartialEq, Eq, ElementDeserialize, ElementSerialize)]
pub struct Feature {
  /// Element-specific attributes
  pub attributes: FeatureAttributes,
  /// Element-specific content
  pub content: String,
}
