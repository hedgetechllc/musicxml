use super::{AccidentalText, DisplayText};
use musicxml_internal::*;
use musicxml_macros::*;

#[derive(Debug, Default, PartialEq, Eq, ContentDeserialize, ContentSerialize)]
pub struct NoteheadTextContents {
  pub display_text: Option<DisplayText>,
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
