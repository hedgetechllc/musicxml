use super::{Assess, OtherListen, Wait};
use musicxml_internal::*;
use musicxml_macros::*;

#[derive(Debug, PartialEq, Eq, ElementDeserialize, ElementSerialize)]
pub enum ListenContents {
  Assess(Assess),
  Wait(Wait),
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
