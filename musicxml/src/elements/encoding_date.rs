use crate::datatypes::YyyyMmDd;
use musicxml_internal::*;
use musicxml_macros::*;

/// The [EncodingDate] element specifies the date of the digital encoding.
#[derive(Debug, PartialEq, Eq, ElementDeserialize, ElementSerialize)]
#[rename("encoding-date")]
pub struct EncodingDate {
  /// Element-specific attributes
  pub attributes: (),
  /// Element-specific content
  pub content: YyyyMmDd,
}
