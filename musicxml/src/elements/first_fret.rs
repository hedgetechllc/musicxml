use crate::datatypes::{LeftRight, PositiveInteger, Token};
use musicxml_internal::*;
use musicxml_macros::*;

/// Attributes pertaining to the [FirstFret] element.
#[derive(Debug, Default, PartialEq, Eq, AttributeDeserialize, AttributeSerialize)]
pub struct FirstFretAttributes {
  /// Indicates whether the text appears to the left or right of the frame.
  pub location: Option<LeftRight>,
  /// Indicates how the first fret is represented in the fret diagram.
  pub text: Option<Token>,
}

/// The [FirstFret] element indicates which fret is shown in the top space of the frame.
/// 
/// It is fret 1 if the element is not present.
#[derive(Debug, PartialEq, Eq, ElementDeserialize, ElementSerialize)]
#[rename("first-fret")]
pub struct FirstFret {
  /// Element-specific attributes
  pub attributes: FirstFretAttributes,
  /// Element-specific content
  pub content: PositiveInteger,
}
