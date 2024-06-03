use musicxml_internal::*;
use musicxml_macros::*;

/// The [Cue] element indicates the presence of a cue note.
///
/// ![Cue](cue.png)
///
/// In MusicXML, a cue note is a silent note with no playback. Normal notes that play can be specified as cue size using the [Type][super::Type] element.
/// A cue note that is specified as full size using the [Type][super::Type] element will still remain silent.
#[derive(Debug, PartialEq, Eq, ElementDeserialize, ElementSerialize)]
pub struct Cue {
  /// Element-specific attributes
  pub attributes: (),
  /// Element-specific content
  pub content: (),
}
