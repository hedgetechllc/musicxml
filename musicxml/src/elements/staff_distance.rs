use crate::datatypes::Tenths;
use alloc::string::String;
use musicxml_internal::*;
use musicxml_macros::*;

/// The [StaffDistance] element represents the vertical distance from the bottom line of the previous staff in this system to the top line of the current staff.
#[derive(Debug, PartialEq, Eq, ElementDeserialize, ElementSerialize)]
#[rename("staff-distance")]
pub struct StaffDistance {
  /// Element-specific attributes
  pub attributes: (),
  /// Element-specific content
  pub content: Tenths,
}
