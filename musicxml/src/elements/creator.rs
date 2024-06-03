use crate::datatypes::Token;
use musicxml_internal::*;
use musicxml_macros::*;

/// Attributes pertaining to the [Creator] element.
#[derive(Debug, Default, PartialEq, Eq, AttributeDeserialize, AttributeSerialize)]
pub struct CreatorAttributes {
  /// Distinguishes different creative contributions. Thus there can be multiple [Creator] elements
  /// within an [Identification][super::Identification] element. Standard values are composer, lyricist, and arranger.
  /// Other values may be used for different types of creative roles. This attribute should usually be used even
  /// if there is just a single [Creator] element.
  pub r#type: Option<Token>,
}

/// The [Creator] element describes the creators of the score.
///
/// This is similar to the Dublin Core creator element. The MusicXML format does not use the creator / contributor distinction from Dublin Core.
#[derive(Debug, PartialEq, Eq, ElementDeserialize, ElementSerialize)]
pub struct Creator {
  /// Element-specific attributes
  pub attributes: CreatorAttributes,
  /// Element-specific content
  pub content: String,
}
