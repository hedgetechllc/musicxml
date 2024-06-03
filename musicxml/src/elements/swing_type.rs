use crate::datatypes::SwingTypeValue;
use musicxml_internal::*;
use musicxml_macros::*;

/// The [SwingType] element specifies the note type, either eighth or 16th, to which the [First][super::First] to [Second][super::Second] ratio is applied.
/// 
/// The value is eighth if this element is not present.
#[derive(Debug, PartialEq, Eq, ElementDeserialize, ElementSerialize)]
#[rename("swing-type")]
pub struct SwingType {
  /// Element-specific attributes
  pub attributes: (),
  /// Element-specific content
  pub content: SwingTypeValue,
}
