use crate::datatypes::PositiveInteger;
use musicxml_internal::*;
use musicxml_macros::*;

/// [Staff] assignment is only needed for music notated on multiple staves.
/// 
/// Staff values are numbers, with 1 referring to the top-most staff in a part.
#[derive(Debug, PartialEq, Eq, ElementDeserialize, ElementSerialize)]
pub struct Staff {
  /// Element-specific attributes
  pub attributes: (),
  /// Element-specific content
  pub content: PositiveInteger,
}
