use crate::datatypes::NonNegativeInteger;
use alloc::string::String;
use musicxml_internal::*;
use musicxml_macros::*;

/// The [StaffLines] element specifies the number of lines and is usually used for a non 5-line staff.
///
/// If the [StaffLines] element is present, the appearance of each line may be individually specified with a [LineDetail][super::LineDetail] element.
#[derive(Debug, PartialEq, Eq, ElementDeserialize, ElementSerialize)]
#[rename("staff-lines")]
pub struct StaffLines {
  /// Element-specific attributes
  pub attributes: (),
  /// Element-specific content
  pub content: NonNegativeInteger,
}
