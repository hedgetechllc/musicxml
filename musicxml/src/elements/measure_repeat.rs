use crate::datatypes::{PositiveInteger, PositiveIntegerOrEmpty, StartStop};
use musicxml_internal::*;
use musicxml_macros::*;

/// Attributes pertaining to the [MeasureRepeat] element.
#[derive(Debug, Default, PartialEq, Eq, AttributeDeserialize, AttributeSerialize)]
pub struct MeasureRepeatAttributes {
  /// Indicates the starting or stopping point of the section displaying the measure repeat symbols.
  pub r#type: Option<StartStop>,
  /// Specifies the number of slashes to use in the symbol. The value is 1 if not specified.
  pub slashes: Option<PositiveInteger>,
}

/// The [MeasureRepeat] element is used for both single and multiple measure repeats.
///
/// ![MeasureRepeat](https://hedgetechllc.github.io/musicxml/musicxml/elements/measure-repeat.png)
///
/// The text of the element indicates the number of measures to be repeated in a single pattern. The text of the element is ignored when the `type` is stop.
///
/// The stop type indicates the first measure where the repeats are no longer displayed. Both the start and the stop of the measures being repeated should be
/// specified unless the repeats are displayed through the end of the part.
///
/// The [MeasureRepeat] element specifies a notation style for repetitions. The actual music being repeated needs to be repeated within each measure of the MusicXML file.
/// This element specifies the notation that indicates the repeat.
#[derive(Debug, PartialEq, Eq, ElementDeserialize, ElementSerialize)]
#[rename("measure-repeat")]
pub struct MeasureRepeat {
  /// Element-specific attributes
  pub attributes: MeasureRepeatAttributes,
  /// Element-specific content
  pub content: PositiveIntegerOrEmpty,
}
