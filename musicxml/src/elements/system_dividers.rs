use super::{LeftDivider, RightDivider};
use musicxml_internal::*;
use musicxml_macros::*;

#[derive(Debug, PartialEq, Eq, ContentDeserialize, ContentSerialize)]
pub struct SystemDividersContents {
  pub left_divider: LeftDivider,
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
