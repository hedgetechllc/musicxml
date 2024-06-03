use super::{
  MeasureLayout, MeasureNumbering, PageLayout, PartAbbreviationDisplay, PartNameDisplay, StaffLayout, SystemLayout,
};
use crate::datatypes::{Id, PositiveInteger, Tenths, Token, YesNo};
use musicxml_internal::*;
use musicxml_macros::*;

/// Attributes pertaining to the [Print] element.
#[derive(Debug, Default, PartialEq, Eq, AttributeDeserialize, AttributeSerialize)]
pub struct PrintAttributes {
  /// The number of blank pages to insert before the current measure. It is ignored if `new_page` is not "yes".
  /// These blank pages have no music, but may have text or images specified by the [Credit][super::Credit] element.
  /// This is used to allow a combination of pages that are all text, or all text and images, together with pages of music.
  pub blank_page: Option<PositiveInteger>,
  /// Specifies an ID that is unique to the entire document.
  pub id: Option<Id>,
  /// Indicates whether to force a page break, or to force the current music onto the same page as the preceding music.
  /// Normally this is the first music data within a measure. If used in multi-part music, the attributes should be placed
  /// in the same positions within each part, or the results are undefined.
  pub new_page: Option<YesNo>,
  /// Indicates whether to force a system break, or to force the current music onto the same system as the preceding music.
  /// Normally this is the first music data within a measure. If used in multi-part music, the attributes should be placed
  /// in the same positions within each part, or the results are undefined.
  pub new_system: Option<YesNo>,
  /// Sets the number of a new page. It is ignored if `new_page` is not "yes".
  pub page_number: Option<Token>,
  /// Specifies spacing between multiple staves in tenths of staff space. Deprecated as of Version 1.1; the [StaffLayout]
  /// element should be used instead. If both are present, the [StaffLayout] values take priority.
  pub staff_spacing: Option<Tenths>,
}

#[derive(Debug, Default, PartialEq, Eq, ContentDeserialize, ContentSerialize)]
pub struct PrintContents {
  pub page_layout: Option<PageLayout>,
  pub system_layout: Option<SystemLayout>,
  pub staff_layout: Vec<StaffLayout>,
  pub measure_layout: Option<MeasureLayout>,
  pub measure_numbering: Option<MeasureNumbering>,
  pub part_name_display: Option<PartNameDisplay>,
  pub part_abbreviation_display: Option<PartAbbreviationDisplay>,
}

/// The [Print] element contains general printing parameters, including layout elements.
/// 
/// The [PartNameDisplay] and [PartAbbreviationDisplay] elements may also be used here to change how a part name or abbreviation is
/// displayed over the course of a piece. They take effect when the current measure or a succeeding measure starts a new system.
/// 
/// Layout group elements in a [Print] element only apply to the current page, system, or staff. Music that follows continues to take
/// the default values from the layout determined by the [Defaults][super::Defaults] element.
#[derive(Debug, PartialEq, Eq, ElementDeserialize, ElementSerialize)]
pub struct Print {
  /// Element-specific attributes
  pub attributes: PrintAttributes,
  #[flatten]
  /// Element-specific content
  pub content: PrintContents,
}
