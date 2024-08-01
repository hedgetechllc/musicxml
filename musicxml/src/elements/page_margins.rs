use super::{BottomMargin, LeftMargin, RightMargin, TopMargin};
use crate::datatypes::MarginType;
use alloc::{string::String, vec::Vec};
use musicxml_internal::*;
use musicxml_macros::*;

/// Attributes pertaining to the [PageMargins] element.
#[derive(Debug, Default, PartialEq, Eq, AttributeDeserialize, AttributeSerialize)]
pub struct PageMarginsAttributes {
  /// Specifies whether the margins apply to even pages, odd pages, or both.
  /// This attribute is not needed when used as part of a [Print][super::Print] element.
  /// The value is "both" if omitted when used in the [Defaults][super::Defaults] element.
  pub r#type: Option<MarginType>,
}

/// Contents of the [PageMargins] element.
#[derive(Debug, PartialEq, Eq, ContentDeserialize, ContentSerialize)]
pub struct PageMarginsContents {
  /// The [LeftMargin] element specifies the left margin in tenths.
  pub left_margin: LeftMargin,
  /// The [RightMargin] element specifies the right margin in tenths.
  pub right_margin: RightMargin,
  /// The [TopMargin] element specifies the top margin in tenths.
  pub top_margin: TopMargin,
  /// The [BottomMargin] element specifies the bottom margin in tenths.
  pub bottom_margin: BottomMargin,
}

/// The [PageMargins] element specifies page margins in tenths either for both even and odd pages, or via separate odd and even page number values.
#[derive(Debug, PartialEq, Eq, ElementDeserialize, ElementSerialize)]
#[rename("page-margins")]
pub struct PageMargins {
  /// Element-specific attributes
  pub attributes: PageMarginsAttributes,
  #[flatten]
  /// Element-specific content
  pub content: PageMarginsContents,
}
