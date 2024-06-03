use super::{AccidentalText, DisplayText};
use crate::datatypes::YesNo;
use musicxml_internal::*;
use musicxml_macros::*;

/// Attributes pertaining to the [PartAbbreviationDisplay] element.
#[derive(Debug, Default, PartialEq, Eq, AttributeDeserialize, AttributeSerialize)]
pub struct PartAbbreviationDisplayAttributes {
  /// Specifies whether or not to print an object. It is yes if not specified.
  pub print_object: Option<YesNo>,
}

#[derive(Debug, Default, PartialEq, Eq, ContentDeserialize, ContentSerialize)]
pub struct PartAbbreviationDisplayContents {
  pub display_text: Option<DisplayText>,
  pub accidental_text: Option<AccidentalText>,
}

/// The [PartAbbreviationDisplay] element is used for exact formatting of multi-font text in part abbreviations to the left of the system.
/// 
/// The `print_object` attribute can be used to determine what, if anything, is printed at the start of each system.
/// 
/// Formatting specified in the [PartAbbreviationDisplay] element overrides formatting specified in the
/// [PartAbbreviation][super::PartAbbreviation] element.
#[derive(Debug, PartialEq, Eq, ElementDeserialize, ElementSerialize)]
#[rename("part-abbreviation-display")]
pub struct PartAbbreviationDisplay {
  /// Element-specific attributes
  pub attributes: PartAbbreviationDisplayAttributes,
  #[flatten]
  /// Element-specific content
  pub content: PartAbbreviationDisplayContents,
}
