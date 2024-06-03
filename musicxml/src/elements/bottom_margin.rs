use crate::datatypes::Tenths;
use musicxml_internal::*;
use musicxml_macros::*;

/// The [BottomMargin] element specifies the bottom page margin in tenths.
#[derive(Debug, PartialEq, Eq, ElementDeserialize, ElementSerialize)]
#[rename("bottom-margin")]
pub struct BottomMargin {
  /// Element-specific attributes
  pub attributes: (),
  /// Element-specific content
  pub content: Tenths,
}
