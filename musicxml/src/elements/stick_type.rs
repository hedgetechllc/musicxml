use musicxml_internal::*;
use musicxml_macros::*;

/// The [StickType] element represents the shape of pictograms where the material in the stick, mallet, or beater is represented in the pictogram.
#[derive(Debug, PartialEq, Eq, ElementDeserialize, ElementSerialize)]
#[rename("stick-type")]
pub struct StickType {
  /// Element-specific attributes
  pub attributes: (),
  /// Element-specific content
  pub content: crate::datatypes::StickType,
}
