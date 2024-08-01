use crate::datatypes::RotationDegrees;
use alloc::string::String;
use musicxml_internal::*;
use musicxml_macros::*;

/// The [Elevation] and [Pan][super::Pan] elements allow placing of sound in a 3-D space relative to the listener.
///
/// Both are expressed in degrees ranging from -180 to 180. For [Elevation], 0 is level with the listener, 90 is directly above, and -90 is directly below.
#[derive(Debug, PartialEq, Eq, ElementDeserialize, ElementSerialize)]
pub struct Elevation {
  /// Element-specific attributes
  pub attributes: (),
  /// Element-specific content
  pub content: RotationDegrees,
}
