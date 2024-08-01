use super::{LeftMargin, RightMargin};
use alloc::{string::String, vec::Vec};
use musicxml_internal::*;
use musicxml_macros::*;

/// Contents of the [SystemMargins] element.
#[derive(Debug, PartialEq, Eq, ContentDeserialize, ContentSerialize)]
pub struct SystemMarginsContents {
  /// The [LeftMargin] element specifies the left system margin.
  pub left_margin: LeftMargin,
  /// The [RightMargin] element specifies the right system margin.
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
