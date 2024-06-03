use musicxml_internal::*;
use musicxml_macros::*;

/// The [TimeRelation] element indicates the symbol used to represent the interchangeable aspect of dual time signatures,
/// as specified in the [TimeRelation][crate::datatypes::TimeRelation] type.
#[derive(Debug, PartialEq, Eq, ElementDeserialize, ElementSerialize)]
#[rename("time-relation")]
pub struct TimeRelation {
  /// Element-specific attributes
  pub attributes: (),
  /// Element-specific content
  pub content: crate::datatypes::TimeRelation,
}
