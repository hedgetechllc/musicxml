use super::{LeftMargin, RightMargin};
use musicxml_internal::*;
use musicxml_macros::*;

#[derive(Debug, PartialEq, Eq, ContentDeserialize, ContentSerialize)]
pub struct SystemMarginsContents {
  pub left_margin: LeftMargin,
  pub right_margin: RightMargin,
}

/// System margins are relative to the page margins.
///
/// Positive values indent and negative values reduce the margin size.
#[derive(Debug, PartialEq, Eq, ElementDeserialize, ElementSerialize)]
#[rename("system-margins")]
pub struct SystemMargins {
  /// Element-specific attributes
  pub attributes: (),
  #[flatten]
  /// Element-specific content
  pub content: SystemMarginsContents,
}
