use crate::datatypes::Fifths;
use musicxml_internal::*;
use musicxml_macros::*;

/// The [NumeralFifths] element specifies the key in the same way as the [Fifths] element.
#[derive(Debug, PartialEq, Eq, ElementDeserialize, ElementSerialize)]
#[rename("numeral-fifths")]
pub struct NumeralFifths {
  /// Element-specific attributes
  pub attributes: (),
  /// Element-specific content
  pub content: Fifths,
}
