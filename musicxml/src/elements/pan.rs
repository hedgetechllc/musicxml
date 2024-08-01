use crate::datatypes::RotationDegrees;
use alloc::string::String;
use musicxml_internal::*;
use musicxml_macros::*;

/// The [Pan] and [Elevation][super::Elevation] elements allow placing of sound in a 3-D space relative to the listener.
///
/// Both are expressed in degrees ranging from -180 to 180. For [Pan], 0 is straight ahead, -90 is hard left, 90 is hard right,
/// and -180 and 180 are directly behind the listener.
#[derive(Debug, PartialEq, Eq, ElementDeserialize, ElementSerialize)]
pub struct Pan {
  /// Element-specific attributes
  pub attributes: (),
  /// Element-specific content
  pub content: RotationDegrees,
}
