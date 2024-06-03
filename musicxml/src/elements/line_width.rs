use crate::datatypes::{LineWidthType, Tenths};
use musicxml_internal::*;
use musicxml_macros::*;

/// Attributes pertaining to the [LineWidth] element.
#[derive(Debug, PartialEq, Eq, AttributeDeserialize, AttributeSerialize)]
pub struct LineWidthAttributes {
  /// The type of line whose width is being defined.
  pub r#type: LineWidthType,
}

/// The [LineWidth] element indicates the width of a specific line type in tenths.
#[derive(Debug, PartialEq, Eq, ElementDeserialize, ElementSerialize)]
#[rename("line-width")]
pub struct LineWidth {
  /// Element-specific attributes
  pub attributes: LineWidthAttributes,
  /// Element-specific content
  pub content: Tenths,
}
