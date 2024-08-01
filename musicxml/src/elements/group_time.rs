use alloc::string::String;
use musicxml_internal::*;
use musicxml_macros::*;

/// The [GroupTime] element indicates that the displayed time signatures should stretch across all parts and staves in the group.
#[derive(Debug, PartialEq, Eq, ElementDeserialize, ElementSerialize)]
#[rename("group-time")]
pub struct GroupTime {
  /// Element-specific attributes
  pub attributes: (),
  /// Element-specific content
  pub content: (),
}
