use musicxml_internal::*;
use musicxml_macros::*;

/// The [WorkTitle] element specifies the title of a work, not including its opus or other work number.
#[derive(Debug, PartialEq, Eq, ElementDeserialize, ElementSerialize)]
#[rename("work-title")]
pub struct WorkTitle {
  /// Element-specific attributes
  pub attributes: (),
  /// Element-specific content
  pub content: String,
}
