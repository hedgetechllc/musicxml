use crate::datatypes::NonNegativeDecimal;
use alloc::{string::String, vec::Vec};
use musicxml_internal::*;
use musicxml_macros::*;

/// Attributes pertaining to the [StaffSize] element.
#[derive(Debug, Default, PartialEq, Eq, AttributeDeserialize, AttributeSerialize)]
pub struct StaffSizeAttributes {
  /// Specifies the percentage scaling that applies to the notation. Values less that 100 make the notation smaller while values over 100 make the notation larger.
  pub scaling: Option<NonNegativeDecimal>,
}

/// The [StaffSize] element indicates how large a staff space is on this staff, expressed as a percentage of the work's default scaling.
///
/// Values less than 100 make the staff space smaller while values over 100 make the staff space larger. A [StaffType][super::StaffType] of cue, ossia,
/// or editorial implies a [StaffSize] of less than 100, but the exact value is implementation-dependent unless specified here. Staff size affects staff height only,
/// not the relationship of the staff to the left and right margins.
///
/// In some cases, a [StaffSize] different than 100 also scales the notation on the staff, such as with a cue staff. In other cases, such as percussion staves,
/// the lines may be more widely spaced without scaling the notation on the staff. The scaling attribute allows these two cases to be distinguished.
#[derive(Debug, PartialEq, Eq, ElementDeserialize, ElementSerialize)]
#[rename("staff-size")]
pub struct StaffSize {
  /// Element-specific attributes
  pub attributes: StaffSizeAttributes,
  /// Element-specific content
  pub content: NonNegativeDecimal,
}
