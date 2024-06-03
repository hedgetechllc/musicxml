use super::{Assess, OtherListen, Wait};
use musicxml_internal::*;
use musicxml_macros::*;

/// The [ListenContents] element specifies all possible options available for use in a [Listen] element.
#[derive(Debug, PartialEq, Eq, ElementDeserialize, ElementSerialize)]
pub enum ListenContents {
  /// The [Assess] element specifies the assessment of a performance.
  Assess(Assess),
  /// The [Wait] element specifies the amount of time to wait before continuing.
  Wait(Wait),
  /// The [OtherListen] element specifies other types of listening.
  #[rename("other-listen")]
  OtherListen(OtherListen),
}

/// The [Listen] and [Listening][super::Listening] elements specify different ways that a score-following or machine-listening application
/// can interact with a performer.
///
/// The [Listen] element handles interactions that are specific to a note. If multiple child elements of the same type are present,
/// they should have distinct `player` and/or `time_only` attributes.
#[derive(Debug, PartialEq, Eq, ElementDeserialize, ElementSerialize)]
pub struct Listen {
  /// Element-specific attributes
  pub attributes: (),
  /// Element-specific content
  pub content: Vec<ListenContents>,
}
