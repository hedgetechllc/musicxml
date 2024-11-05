use super::{AccidentalText, DisplayText};
use crate::datatypes::YesNo;
use alloc::{string::String, vec::Vec};
use musicxml_internal::*;
use musicxml_macros::*;

/// Attributes pertaining to the [GroupNameDisplay] element.
#[derive(Debug, Default, PartialEq, Eq, AttributeDeserialize, AttributeSerialize)]
pub struct GroupNameDisplayAttributes {
  /// Specifies whether or not to print an object. It is yes if not specified.
  pub print_object: Option<YesNo>,
}

/// Contents of the [GroupNameDisplay] element.
#[derive(Debug, PartialEq, Eq, ContentDeserialize, ContentSerialize)]
pub struct GroupNameDisplayContents {
  /// The [DisplayText] element specifies the text to display for the group name.
  pub display_text: Vec<DisplayText>,
  /// The [AccidentalText] element specifies the accidental to display for the group name.
  pub accidental_text: Vec<AccidentalText>,
}

/// The [GroupNameDisplay] element is used for exact formatting of multi-font text in group names to the left of the system.
///
/// The `print_object` attribute can be used to determine what, if anything, is printed at the start of each system.
///
/// Formatting specified in the [GroupNameDisplay] element overrides formatting specified in the [GroupName][super::GroupName] element.
#[derive(Debug, PartialEq, Eq, ElementDeserialize, ElementSerialize)]
#[rename("group-name-display")]
pub struct GroupNameDisplay {
  /// Element-specific attributes
  pub attributes: GroupNameDisplayAttributes,
  #[flatten]
  /// Element-specific content
  pub content: GroupNameDisplayContents,
}
