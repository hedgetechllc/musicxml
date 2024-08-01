use super::{Duration, Footnote, Level, Staff, Voice};
use alloc::{string::String, vec::Vec};
use musicxml_internal::*;
use musicxml_macros::*;

/// Contents of the [Forward] element.
#[derive(Debug, PartialEq, Eq, ContentDeserialize, ContentSerialize)]
pub struct ForwardContents {
  /// The [Duration] element specifies the duration of the forward element.
  pub duration: Duration,
  /// The [Footnote] element specifies editorial information or lyrics content.
  pub footnote: Option<Footnote>,
  /// The [Level] element specifies the editorial level of a score or part.
  pub level: Option<Level>,
  /// The [Voice] element specifies the playback voice.
  pub voice: Option<Voice>,
  /// The [Staff] element specifies the staff for a forward element.
  pub staff: Option<Staff>,
}

/// The [Backup][super::Backup] and [Forward] elements are required to coordinate multiple voices in one part,
/// including music on multiple staves.
///
/// The [Forward] element is generally used within voices and staves. [Duration] values should always be positive, and should not cross
/// measure boundaries or mid-measure changes in the [Divisions][super::Divisions] value.
#[derive(Debug, PartialEq, Eq, ElementDeserialize, ElementSerialize)]
pub struct Forward {
  /// Element-specific attributes
  pub attributes: (),
  #[flatten]
  /// Element-specific content
  pub content: ForwardContents,
}
