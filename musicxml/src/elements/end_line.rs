use alloc::string::String;
use musicxml_internal::*;
use musicxml_macros::*;

/// The [EndLine] element comes from RP-017 for Standard MIDI File Lyric meta-events.
///
/// It facilitates lyric display for Karaoke and similar applications.
#[derive(Debug, PartialEq, Eq, ElementDeserialize, ElementSerialize)]
#[rename("end-line")]
pub struct EndLine {
  /// Element-specific attributes
  pub attributes: (),
  /// Element-specific content
  pub content: (),
}
