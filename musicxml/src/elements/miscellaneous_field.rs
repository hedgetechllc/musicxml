use crate::datatypes::Token;
use musicxml_internal::*;
use musicxml_macros::*;

/// Attributes pertaining to the [MiscellaneousField] element.
#[derive(Debug, PartialEq, Eq, AttributeDeserialize, AttributeSerialize)]
pub struct MiscellaneousFieldAttributes {
  /// Indicates the type of metadata the element content represents.
  pub name: Token,
}

/// If a program has other metadata not yet supported in the MusicXML format, each type of metadata can go in a [MiscellaneousField] element.
#[derive(Debug, PartialEq, Eq, ElementDeserialize, ElementSerialize)]
#[rename("miscellaneous-field")]
pub struct MiscellaneousField {
  /// Element-specific attributes
  pub attributes: MiscellaneousFieldAttributes,
  /// Element-specific content
  pub content: String,
}
