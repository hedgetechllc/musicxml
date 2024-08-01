use super::{AccidentalText, DisplayText};
use crate::datatypes::YesNo;
use alloc::{string::String, vec::Vec};
use musicxml_internal::*;
use musicxml_macros::*;

/// Attributes pertaining to the [PartNameDisplay] element.
#[derive(Debug, Default, PartialEq, Eq, AttributeDeserialize, AttributeSerialize)]
pub struct PartNameDisplayAttributes {
  /// Specifies whether or not to print an object. It is yes if not specified.
  pub print_object: Option<YesNo>,
}

/// Contents of the [PartNameDisplay] element.
#[derive(Debug, Default, PartialEq, Eq, ContentDeserialize, ContentSerialize)]
pub struct PartNameDisplayContents {
  /// The [DisplayText] element specifies the text to display for the part name.
  pub display_text: Option<DisplayText>,
  /// The [AccidentalText] element specifies the accidental to display for the part name.
  pub accidental_text: Option<AccidentalText>,
}

/// The [PartNameDisplay] element is used for exact formatting of multi-font text in part names to the left of the system.
///
/// The `print_object` attribute can be used to determine what, if anything, is printed at the start of each system.
///
/// Formatting specified in the [PartNameDisplay] element overrides formatting specified in the [PartName][super::PartName] element.
#[derive(Debug, PartialEq, Eq, ElementDeserialize, ElementSerialize)]
#[rename("part-name-display")]
pub struct PartNameDisplay {
  /// Element-specific attributes
  pub attributes: PartNameDisplayAttributes,
  #[flatten]
  /// Element-specific content
  pub content: PartNameDisplayContents,
}
