use crate::datatypes::{CancelLocation, Fifths};
use musicxml_internal::*;
use musicxml_macros::*;

/// Attributes pertaining to the [Cancel] element.
#[derive(Debug, Default, PartialEq, Eq, AttributeDeserialize, AttributeSerialize)]
pub struct CancelAttributes {
  /// Indicates where the cancellation appears relative to the new key signature. It is "left" if not specified.
  pub location: Option<CancelLocation>,
}

/// The [Cancel] element indicates that the old key signature should be cancelled before the new one appears.
/// 
/// ![Cancel](cancel.png)
/// 
/// This will always happen when changing to C major or A minor and need not be specified then.
/// The [Cancel] element value matches the fifths value of the cancelled key signature
/// (e.g., a cancel of -2 will provide an explicit cancellation for changing from B flat major to F major).
#[derive(Debug, PartialEq, Eq, ElementDeserialize, ElementSerialize)]
pub struct Cancel {
  /// Element-specific attributes
  pub attributes: CancelAttributes,
  /// Element-specific content
  pub content: Fifths,
}
