use super::{Duration, Footnote, Level, Staff, Voice};
use musicxml_internal::*;
use musicxml_macros::*;

#[derive(Debug, PartialEq, Eq, ContentDeserialize, ContentSerialize)]
pub struct ForwardContents {
  pub duration: Duration,
  pub footnote: Option<Footnote>,
  pub level: Option<Level>,
  pub voice: Option<Voice>,
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
