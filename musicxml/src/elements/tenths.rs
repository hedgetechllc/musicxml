use musicxml_internal::*;
use musicxml_macros::*;

/// The [Tenths] element contains the number of tenths that correspond to the given number of millimeters within the [Scaling][super::Scaling] element formula.
/// 
/// Setting this to 40 allows the [Millimeters][super::Millimeters] element to specify the size of a 5-line staff.
#[derive(Debug, PartialEq, Eq, ElementDeserialize, ElementSerialize)]
pub struct Tenths {
  /// Element-specific attributes
  pub attributes: (),
  /// Element-specific content
  pub content: crate::datatypes::Tenths,
}
