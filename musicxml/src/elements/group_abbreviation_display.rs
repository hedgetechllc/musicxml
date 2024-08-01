use super::{AccidentalText, DisplayText};
use crate::datatypes::YesNo;
use alloc::{string::String, vec::Vec};
use musicxml_internal::*;
use musicxml_macros::*;

/// Attributes pertaining to the [GroupAbbreviationDisplay] element.
#[derive(Debug, Default, PartialEq, Eq, AttributeDeserialize, AttributeSerialize)]
pub struct GroupAbbreviationDisplayAttributes {
  /// Specifies whether or not to print an object. It is yes if not specified.
  pub print_object: Option<YesNo>,
}

/// Contents of the [GroupAbbreviationDisplay] element.
#[derive(Debug, Default, PartialEq, Eq, ContentDeserialize, ContentSerialize)]
pub struct GroupAbbreviationDisplayContents {
  /// The [DisplayText] element specifies the text of the group abbreviation.
  pub display_text: Vec<DisplayText>,
  /// The [AccidentalText] element specifies the accidental of the group abbreviation.
  pub accidental_text: Vec<AccidentalText>,
}

/// The [GroupAbbreviationDisplay] element is used for exact formatting of multi-font text in group abbreviations to the left of the system.
///
/// The `print_object` attribute can be used to determine what, if anything, is printed at the start of each system.
///
/// Formatting specified in the [GroupAbbreviationDisplay] element overrides formatting specified in the
/// [GroupAbbreviation][super::GroupAbbreviation] element.
#[derive(Debug, PartialEq, Eq, ElementDeserialize, ElementSerialize)]
#[rename("group-abbreviation-display")]
pub struct GroupAbbreviationDisplay {
  /// Element-specific attributes
  pub attributes: GroupAbbreviationDisplayAttributes,
  #[flatten]
  /// Element-specific content
  pub content: GroupAbbreviationDisplayContents,
}
