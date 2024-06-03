use super::StaffDistance;
use crate::datatypes::StaffNumber;
use musicxml_internal::*;
use musicxml_macros::*;

/// Attributes pertaining to the [StaffLayout] element.
#[derive(Debug, Default, PartialEq, Eq, AttributeDeserialize, AttributeSerialize)]
pub struct StaffLayoutAttributes {
  /// Refers to staff numbers within the part, from top to bottom on the system. A value of 1 is used if not present.
  pub number: Option<StaffNumber>,
}

/// Contents of the [StaffLayout] element.
#[derive(Debug, Default, PartialEq, Eq, ContentDeserialize, ContentSerialize)]
pub struct StaffLayoutContents {
  /// The [StaffDistance] element specifies the vertical distance from the bottom line of the previous staff in this system to the top line of the staff specified by the `number` attribute.
  pub staff_distance: Option<StaffDistance>,
}

/// The [StaffLayout] element includes the vertical distance from the bottom line of the previous staff in this system to the
/// top line of the staff specified by the `number` attribute.
///
/// When used in the [Defaults][super::Defaults] element, the values apply to all systems in all parts. When used in the [Print][super::Print] element,
/// the values apply to the current system only. This value is ignored for the first staff in a system.
#[derive(Debug, PartialEq, Eq, ElementDeserialize, ElementSerialize)]
#[rename("staff-layout")]
pub struct StaffLayout {
  /// Element-specific attributes
  pub attributes: StaffLayoutAttributes,
  #[flatten]
  /// Element-specific content
  pub content: StaffLayoutContents,
}
