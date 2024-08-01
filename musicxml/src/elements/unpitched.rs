use super::{DisplayOctave, DisplayStep};
use alloc::{string::String, vec::Vec};
use musicxml_internal::*;
use musicxml_macros::*;

/// Contents of the [Unpitched] element.
#[derive(Debug, PartialEq, Eq, ContentDeserialize, ContentSerialize)]
pub struct UnpitchedContents {
  /// The [DisplayStep] element specifies the pitch step of the unpitched note.
  pub display_step: DisplayStep,
  /// The [DisplayOctave] element specifies the pitch octave of the unpitched note.
  pub display_octave: DisplayOctave,
}

/// The [Unpitched] element represents notes that are notated on the staff but lack definite pitch, such as unpitched percussion and speaking voice.
///
/// ![Unpitched](https://hedgetechllc.github.io/musicxml/musicxml/elements/unpitched.png)
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
