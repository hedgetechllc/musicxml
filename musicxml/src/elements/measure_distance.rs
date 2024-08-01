use crate::datatypes::Tenths;
use alloc::string::String;
use musicxml_internal::*;
use musicxml_macros::*;

/// The [MeasureDistance] element specifies the horizontal distance from the previous measure.
///
/// This value is only used for systems where there is horizontal whitespace in the middle of a system, as in systems with codas.
/// To specify the measure width, use the `width` attribute of the [Measure][super::Measure] element.
#[derive(Debug, PartialEq, Eq, ElementDeserialize, ElementSerialize)]
#[rename("measure-distance")]
pub struct MeasureDistance {
  /// Element-specific attributes
  pub attributes: (),
  /// Element-specific content
  pub content: Tenths,
}
