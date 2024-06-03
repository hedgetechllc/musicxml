use musicxml_internal::*;
use musicxml_macros::*;

/// The [WorkNumber] element specifies the number of a work, such as its opus number.
#[derive(Debug, PartialEq, Eq, ElementDeserialize, ElementSerialize)]
#[rename("work-number")]
pub struct WorkNumber {
  /// Element-specific attributes
  pub attributes: (),
  /// Element-specific content
  pub content: String,
}
