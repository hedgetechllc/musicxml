use crate::datatypes::{BackwardForward, NonNegativeInteger, Winged, YesNo};
use musicxml_internal::*;
use musicxml_macros::*;

/// Attributes pertaining to the [Repeat] element.
#[derive(Debug, PartialEq, Eq, AttributeDeserialize, AttributeSerialize)]
pub struct RepeatAttributes {
  /// The start of the repeat has a forward direction while the end of the repeat has a backward direction.
  pub direction: BackwardForward,
  /// Indicates if the repeats are played after a jump due to a da capo or dal segno.
  /// It is only used with backward repeats that are not part of an ending.
  pub after_jump: Option<YesNo>,
  /// Indicates the number of times the repeated section is played.
  /// It is only used with backward repeats that are not part of an ending.
  pub times: Option<NonNegativeInteger>,
  /// Indicates whether the repeat has winged extensions that appear above and below the barline.
  pub winged: Option<Winged>,
}

/// The [Repeat] element represents repeat marks.
///
/// ![Repeat](repeat.png)
#[derive(Debug, PartialEq, Eq, ElementDeserialize, ElementSerialize)]
pub struct Repeat {
  /// Element-specific attributes
  pub attributes: RepeatAttributes,
  /// Element-specific content
  pub content: (),
}
