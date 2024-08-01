use super::{AccidentalText, DisplayText};
use alloc::{string::String, vec::Vec};
use musicxml_internal::*;
use musicxml_macros::*;

/// Contents of the [NoteheadText] element.
#[derive(Debug, Default, PartialEq, Eq, ContentDeserialize, ContentSerialize)]
pub struct NoteheadTextContents {
  /// The [DisplayText] element specifies the text that is displayed inside a notehead.
  pub display_text: Option<DisplayText>,
  /// The [AccidentalText] element specifies the accidental that is displayed inside a notehead.
  pub accidental_text: Option<AccidentalText>,
}

/// The [NoteheadText] element represents text that is displayed inside a notehead, as is done in some educational music.
///
/// It is not needed for the numbers used in tablature or jianpu notation. The presence of a TAB or jianpu clefs is sufficient to
/// indicate that numbers are used. The [DisplayText] and [AccidentalText] elements allow display of fully formatted text and accidentals.
#[derive(Debug, PartialEq, Eq, ElementDeserialize, ElementSerialize)]
#[rename("notehead-text")]
pub struct NoteheadText {
  /// Element-specific attributes
  pub attributes: (),
  #[flatten]
  /// Element-specific content
  pub content: NoteheadTextContents,
}
