use alloc::string::String;
use musicxml_internal::*;
use musicxml_macros::*;

/// The [ArrowStyle] element represents the style of an arrow, using Unicode arrow terminology.
#[derive(Debug, PartialEq, Eq, ElementDeserialize, ElementSerialize)]
#[rename("arrow-style")]
pub struct ArrowStyle {
  /// Element-specific attributes
  pub attributes: (),
  /// Element-specific content
  pub content: crate::datatypes::ArrowStyle,
}
