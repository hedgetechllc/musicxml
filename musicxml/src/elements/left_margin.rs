use crate::datatypes::Tenths;
use alloc::string::String;
use musicxml_internal::*;
use musicxml_macros::*;

/// The [LeftMargin] element specifies the left margin for the parent element in tenths.
#[derive(Debug, PartialEq, Eq, ElementDeserialize, ElementSerialize)]
#[rename("left-margin")]
pub struct LeftMargin {
  /// Element-specific attributes
  pub attributes: (),
  /// Element-specific content
  pub content: Tenths,
}
