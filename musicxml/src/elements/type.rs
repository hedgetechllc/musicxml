use crate::datatypes::{NoteTypeValue, SymbolSize};
use musicxml_internal::*;
use musicxml_macros::*;

/// Attributes pertaining to the [Type] element.
#[derive(Debug, Default, PartialEq, Eq, AttributeDeserialize, AttributeSerialize)]
pub struct TypeAttributes {
  /// Specifies the symbol size to use for an editorial indication. If not specified, it is left to application defaults.
  pub size: Option<SymbolSize>,
}

/// The [Type] element indicates the graphic note type.
/// 
/// Values range from 1024th to maxima.
#[derive(Debug, PartialEq, Eq, ElementDeserialize, ElementSerialize)]
pub struct Type {
  /// Element-specific attributes
  pub attributes: TypeAttributes,
  /// Element-specific content
  pub content: NoteTypeValue,
}
