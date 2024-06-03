use super::{Barre, Fingering, Fret, StringNumber};
use musicxml_internal::*;
use musicxml_macros::*;

#[derive(Debug, PartialEq, Eq, ContentDeserialize, ContentSerialize)]
pub struct FrameNoteContents {
  pub string: StringNumber,
  pub fret: Fret,
  pub fingering: Option<Fingering>,
  pub barre: Option<Barre>,
}

/// The [FrameNote] type represents each note included in the frame.
///
/// An open string will have a fret value of 0, while a muted string will not be associated with a [FrameNote] element.
#[derive(Debug, PartialEq, Eq, ElementDeserialize, ElementSerialize)]
#[rename("frame-note")]
pub struct FrameNote {
  /// Element-specific attributes
  pub attributes: (),
  #[flatten]
  /// Element-specific content
  pub content: FrameNoteContents,
}
