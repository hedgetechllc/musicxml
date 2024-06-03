use super::{PageHeight, PageMargins, PageWidth};
use musicxml_internal::*;
use musicxml_macros::*;

#[derive(Debug, Default, PartialEq, Eq, ContentDeserialize, ContentSerialize)]
pub struct PageLayoutContents {
  pub page_height: Option<PageHeight>,
  pub page_width: Option<PageWidth>,
  pub page_margins: Vec<PageMargins>,
}

/// Page layout can be defined both in score-wide [Defaults][super::Defaults] and in the [Print][super::Print] element.
/// 
/// If no [PageLayout] element is present in the [Defaults][super::Defaults] element, default page layout values are chosen by the application.
/// 
/// When used in the [Print][super::Print] element, the [PageLayout] element affects the appearance of the current page only.
/// All other pages use the default values as determined by the [Defaults][super::Defaults] element. If any child elements are missing
/// from the [PageLayout] element in a [Print][super::Print] element, the values determined by the [Defaults][super::Defaults] element
/// are used there as well.
#[derive(Debug, PartialEq, Eq, ElementDeserialize, ElementSerialize)]
#[rename("page-layout")]
pub struct PageLayout {
  /// Element-specific attributes
  pub attributes: (),
  #[flatten]
  /// Element-specific content
  pub content: PageLayoutContents,
}
