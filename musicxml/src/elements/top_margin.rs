use crate::datatypes::Tenths;
use musicxml_internal::*;
use musicxml_macros::*;

/// The [TopMargin] element specifies the top page margin in tenths.
#[derive(Debug, PartialEq, Eq, ElementDeserialize, ElementSerialize)]
#[rename("top-margin")]
pub struct TopMargin {
  /// Element-specific attributes
  pub attributes: (),
  /// Element-specific content
  pub content: Tenths,
}
