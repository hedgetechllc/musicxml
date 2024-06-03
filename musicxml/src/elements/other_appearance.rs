use crate::datatypes::Token;
use musicxml_internal::*;
use musicxml_macros::*;

/// Attributes pertaining to the [OtherAppearance] element.
#[derive(Debug, PartialEq, Eq, AttributeDeserialize, AttributeSerialize)]
pub struct OtherAppearanceAttributes {
  /// The appearance type being specified.
  pub r#type: Token,
}

/// The [OtherAppearance] element is used to define any graphical settings not yet in the current version of the MusicXML format.
///
/// This allows extended representation, though without application interoperability.
#[derive(Debug, PartialEq, Eq, ElementDeserialize, ElementSerialize)]
#[rename("other-appearance")]
pub struct OtherAppearance {
  /// Element-specific attributes
  pub attributes: OtherAppearanceAttributes,
  /// Element-specific content
  pub content: String,
}
