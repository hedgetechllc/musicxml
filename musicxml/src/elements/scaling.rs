use super::{Millimeters, Tenths};
use musicxml_internal::*;
use musicxml_macros::*;

#[derive(Debug, PartialEq, Eq, ContentDeserialize, ContentSerialize)]
pub struct ScalingContents {
  pub millimeters: Millimeters,
  pub tenths: Tenths,
}

/// Margins, page sizes, and distances are all measured in tenths to keep MusicXML data in a consistent coordinate system as much as possible.
///
/// The translation to absolute units is done with the [Scaling] element, which specifies how many millimeters are equal to how many tenths.
/// For a staff height of 7 mm, [Millimeters] would be set to 7 while [Tenths] is set to 40. The ability to set a formula rather than a single scaling
/// factor helps avoid roundoff errors.
#[derive(Debug, PartialEq, Eq, ElementDeserialize, ElementSerialize)]
pub struct Scaling {
  /// Element-specific attributes
  pub attributes: (),
  #[flatten]
  /// Element-specific content
  pub content: ScalingContents,
}
