use super::{Encoder, EncodingDate, EncodingDescription, Software, Supports};
use musicxml_internal::*;
use musicxml_macros::*;

#[derive(Debug, PartialEq, Eq, ElementDeserialize, ElementSerialize)]
pub enum EncodingContents {
  #[rename("encoding-date")]
  EncodingDate(EncodingDate),
  Encoder(Encoder),
  Software(Software),
  #[rename("encoding-description")]
  EncodingDescription(EncodingDescription),
  Supports(Supports),
}

/// The [Encoding] element contains information about who did the digital encoding, when, with what software, and in what aspects.
#[derive(Debug, PartialEq, Eq, ElementDeserialize, ElementSerialize)]
pub struct Encoding {
  /// Element-specific attributes
  pub attributes: (),
  /// Element-specific content
  pub content: Vec<EncodingContents>,
}
