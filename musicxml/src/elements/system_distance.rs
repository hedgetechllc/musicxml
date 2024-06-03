use crate::datatypes::Tenths;
use musicxml_internal::*;
use musicxml_macros::*;

/// The [SystemDistance] is measured from the bottom line of the previous system to the top line of the current system.
/// 
/// It is ignored for the first system on a page.
#[derive(Debug, PartialEq, Eq, ElementDeserialize, ElementSerialize)]
#[rename("system-distance")]
pub struct SystemDistance {
  /// Element-specific attributes
  pub attributes: (),
  /// Element-specific content
  pub content: Tenths,
}
