use crate::datatypes::PositiveIntegerOrEmpty;
use musicxml_internal::*;
use musicxml_macros::*;

/// The [Ensemble] element is present if performance is intended by an ensemble such as an orchestral section.
///
/// The text of the [Ensemble] element contains the size of the section, or is empty if the ensemble size is not specified.
#[derive(Debug, PartialEq, Eq, ElementDeserialize, ElementSerialize)]
pub struct Ensemble {
  /// Element-specific attributes
  pub attributes: (),
  /// Element-specific content
  pub content: PositiveIntegerOrEmpty,
}
