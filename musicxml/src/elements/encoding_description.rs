use musicxml_internal::*;
use musicxml_macros::*;

/// The [EncodingDescription] element contains descriptive information about the digital encoding
/// that is not provided in the other [Encoding][super::Encoding] child elements.
#[derive(Debug, PartialEq, Eq, ElementDeserialize, ElementSerialize)]
#[rename("encoding-description")]
pub struct EncodingDescription {
  /// Element-specific attributes
  pub attributes: (),
  /// Element-specific content
  pub content: String,
}
