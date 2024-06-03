use super::{Encoder, EncodingDate, EncodingDescription, Software, Supports};
use musicxml_internal::*;
use musicxml_macros::*;

/// The [EncodingContents] element specifies all possible options available for use in an [Encoding] element.
#[derive(Debug, PartialEq, Eq, ElementDeserialize, ElementSerialize)]
pub enum EncodingContents {
  /// The [EncodingDate] element specifies the date of the encoding.
  #[rename("encoding-date")]
  EncodingDate(EncodingDate),
  /// The [Encoder] element specifies the person or organization who did the digital encoding.
  Encoder(Encoder),
  /// The [Software] element specifies the software used for the encoding.
  Software(Software),
  /// The [EncodingDescription] element specifies the description of the encoding.
  #[rename("encoding-description")]
  EncodingDescription(EncodingDescription),
  /// The [Supports] element specifies the supports for the encoding.
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
