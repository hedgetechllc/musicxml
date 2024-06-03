use super::{Barre, Fingering, Fret, StringNumber};
use musicxml_internal::*;
use musicxml_macros::*;

/// Contents of the [FrameNote] element.
#[derive(Debug, PartialEq, Eq, ContentDeserialize, ContentSerialize)]
pub struct FrameNoteContents {
  /// The [StringNumber] element specifies the string number of the note.
  pub string: StringNumber,
  /// The [Fret] element specifies the fret number of the note.
  pub fret: Fret,
  /// The [Fingering] element specifies the fingering of the note.
  pub fingering: Option<Fingering>,
  /// The [Barre] element specifies the barre of the note.
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
