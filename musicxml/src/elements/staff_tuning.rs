use super::{TuningAlter, TuningOctave, TuningStep};
use crate::datatypes::StaffLine;
use musicxml_internal::*;
use musicxml_macros::*;

/// Attributes pertaining to the [StaffTuning] element.
#[derive(Debug, PartialEq, Eq, AttributeDeserialize, AttributeSerialize)]
pub struct StaffTuningAttributes {
  /// Indicates the staff line for this tuning, numbered from bottom to top.
  pub line: StaffLine,
}

/// Contents of the [StaffTuning] element.
#[derive(Debug, PartialEq, Eq, ContentDeserialize, ContentSerialize)]
pub struct StaffTuningContents {
  /// The [TuningStep] element specifies the step of the tuning.
  pub tuning_step: TuningStep,
  /// The [TuningAlter] element specifies the alteration of the tuning.
  pub tuning_alter: Option<TuningAlter>,
  /// The [TuningOctave] element specifies the octave of the tuning.
  pub tuning_octave: TuningOctave,
}

/// The [StaffTuning] element specifies the open, non-capo tuning of the lines on a tablature staff.
///
/// ![StaffTuning](https://hedgetechllc.github.io/musicxml/musicxml/elements/staff-tuning.png)
#[derive(Debug, PartialEq, Eq, ElementDeserialize, ElementSerialize)]
#[rename("staff-tuning")]
pub struct StaffTuning {
  /// Element-specific attributes
  pub attributes: StaffTuningAttributes,
  #[flatten]
  /// Element-specific content
  pub content: StaffTuningContents,
}
