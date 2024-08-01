use crate::datatypes::Tenths;
use alloc::string::String;
use musicxml_internal::*;
use musicxml_macros::*;

/// The [RightMargin] element specifies the right margin for the parent element in tenths.
#[derive(Debug, PartialEq, Eq, ElementDeserialize, ElementSerialize)]
#[rename("right-margin")]
pub struct RightMargin {
  /// Element-specific attributes
  pub attributes: (),
  /// Element-specific content
  pub content: Tenths,
}
