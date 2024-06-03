use musicxml_internal::*;
use musicxml_macros::*;

/// The [EndParagraph] element comes from RP-017 for Standard MIDI File Lyric meta-events.
/// 
/// It facilitates lyric display for Karaoke and similar applications.
#[derive(Debug, PartialEq, Eq, ElementDeserialize, ElementSerialize)]
#[rename("end-paragraph")]
pub struct EndParagraph {
  /// Element-specific attributes
  pub attributes: (),
  /// Element-specific content
  pub content: (),
}
