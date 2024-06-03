use crate::datatypes::StaffLinePosition;
use musicxml_internal::*;
use musicxml_macros::*;

/// [Line] numbers are counted from the bottom of the staff.
/// 
/// They are only needed with the G, F, and C signs in order to position a pitch correctly on the staff.
/// Standard values are 2 for the G sign (treble clef), 4 for the F sign (bass clef), and 3 for the C sign (alto clef).
/// Line values can be used to specify positions outside the staff, such as a C clef positioned in the middle of a grand staff.
#[derive(Debug, PartialEq, Eq, ElementDeserialize, ElementSerialize)]
pub struct Line {
  /// Element-specific attributes
  pub attributes: (),
  /// Element-specific content
  pub content: StaffLinePosition,
}
