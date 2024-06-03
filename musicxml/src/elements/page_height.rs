use crate::datatypes::Tenths;
use musicxml_internal::*;
use musicxml_macros::*;

/// The [PageHeight] element specifies the page height in tenths.
#[derive(Debug, PartialEq, Eq, ElementDeserialize, ElementSerialize)]
#[rename("page-height")]
pub struct PageHeight {
  /// Element-specific attributes
  pub attributes: (),
  /// Element-specific content
  pub content: Tenths,
}
