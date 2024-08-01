use super::{LeftDivider, RightDivider};
use alloc::{string::String, vec::Vec};
use musicxml_internal::*;
use musicxml_macros::*;

/// Contents of the [SystemDividers] element.
#[derive(Debug, PartialEq, Eq, ContentDeserialize, ContentSerialize)]
pub struct SystemDividersContents {
  /// The [LeftDivider] element indicates the presence or absence of a system divider on the left side of the page.
  pub left_divider: LeftDivider,
  /// The [RightDivider] element indicates the presence or absence of a system divider on the right side of the page.
  pub right_divider: RightDivider,
}

/// The [SystemDividers] element indicates the presence or absence of system dividers (also known as system separation marks) between systems displayed on the same page.
///
/// Dividers on the left and right side of the page are controlled by the [LeftDivider] and [RightDivider] elements respectively.
///
/// When used in the [Print][super::Print] element, the [SystemDividers] element affects the dividers that would appear between the current system and the previous system.
#[derive(Debug, PartialEq, Eq, ElementDeserialize, ElementSerialize)]
#[rename("system-dividers")]
pub struct SystemDividers {
  /// Element-specific attributes
  pub attributes: (),
  #[flatten]
  /// Element-specific content
  pub content: SystemDividersContents,
}
