use super::{Offset, OtherListening, Sync};
use alloc::{string::String, vec::Vec};
use musicxml_internal::*;
use musicxml_macros::*;

/// Contents of the [Listening] element.
#[derive(Debug, Default, PartialEq, Eq, ContentDeserialize, ContentSerialize)]
pub struct ListeningContents {
  /// The [Sync] element specifies the synchronization of the score-following or machine-listening data with the music.
  pub sync: Option<Sync>,
  /// The [OtherListening] element specifies other types of listening.
  pub other_listening: Option<OtherListening>,
  /// The [Offset] element is used to indicate that the listening change takes place offset from the current score position.
  pub offset: Option<Offset>,
}

/// The [Listen][super::Listen] and [Listening] elements specify different ways that a score-following or machine-listening application
/// can interact with a performer.
///
/// The [Listening] element handles interactions that change the state of the listening application from the specified point
/// in the performance onward. If multiple child elements of the same type are present, they should have distinct `player` and/or `time_only` attributes.
///
/// The [Offset] element is used to indicate that the listening change takes place offset from the current score position. If the [Listening] element is a
/// child of a [Direction][super::Direction] element, the listening [Offset] element overrides the direction [Offset] element if both elements are present.
///
/// Note that the [Offset] reflects the intended musical position for the change in state. It should not be used to compensate for latency issues in
/// particular hardware configurations.
#[derive(Debug, PartialEq, Eq, ElementDeserialize, ElementSerialize)]
pub struct Listening {
  /// Element-specific attributes
  pub attributes: (),
  #[flatten]
  /// Element-specific content
  pub content: ListeningContents,
}
