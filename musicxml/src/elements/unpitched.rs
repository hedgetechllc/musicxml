use super::{DisplayOctave, DisplayStep};
use musicxml_internal::*;
use musicxml_macros::*;

#[derive(Debug, PartialEq, Eq, ContentDeserialize, ContentSerialize)]
pub struct UnpitchedContents {
  pub display_step: DisplayStep,
  pub display_octave: DisplayOctave,
}

/// The [Unpitched] element represents notes that are notated on the staff but lack definite pitch, such as unpitched percussion and speaking voice.
/// 
/// ![Unpitched](unpitched.png)
/// 
/// If the child elements are not present, the note is placed on the middle line of the staff. This is generally used with a one-line staff.
/// Notes in percussion clef should always use an [Unpitched] element rather than a [Pitch][super::Pitch] element.
#[derive(Debug, PartialEq, Eq, ElementDeserialize, ElementSerialize)]
pub struct Unpitched {
  /// Element-specific attributes
  pub attributes: (),
  #[flatten]
  /// Element-specific content
  pub content: UnpitchedContents,
}
