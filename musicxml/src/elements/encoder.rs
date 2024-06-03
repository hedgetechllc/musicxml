use crate::datatypes::Token;
use musicxml_internal::*;
use musicxml_macros::*;

/// Attributes pertaining to the [Encoder] element.
#[derive(Debug, Default, PartialEq, Eq, AttributeDeserialize, AttributeSerialize)]
pub struct EncoderAttributes {
  /// Standard values are music, words, and arrangement, but other types may be used.
  /// This attribute is only needed when there are multiple [Encoder] elements.
  pub r#type: Option<Token>,
}

/// The [Encoder] element contains information about who did the digital encoding.
#[derive(Debug, PartialEq, Eq, ElementDeserialize, ElementSerialize)]
pub struct Encoder {
  /// Element-specific attributes
  pub attributes: EncoderAttributes,
  /// Element-specific content
  pub content: String,
}
