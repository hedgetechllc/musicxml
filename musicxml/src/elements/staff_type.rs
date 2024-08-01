use alloc::string::String;
use musicxml_internal::*;
use musicxml_macros::*;

/// The [StaffType] element specifies different uses for the staff, as listed in the [StaffType][crate::datatypes::StaffType] data type.
#[derive(Debug, PartialEq, Eq, ElementDeserialize, ElementSerialize)]
#[rename("staff-type")]
pub struct StaffType {
  /// Element-specific attributes
  pub attributes: (),
  /// Element-specific content
  pub content: crate::datatypes::StaffType,
}
