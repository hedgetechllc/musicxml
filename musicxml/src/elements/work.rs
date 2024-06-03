use super::{Opus, WorkNumber, WorkTitle};
use musicxml_internal::*;
use musicxml_macros::*;

#[derive(Debug, Default, PartialEq, Eq, ContentDeserialize, ContentSerialize)]
pub struct WorkContents {
  pub work_number: Option<WorkNumber>,
  pub work_title: Option<WorkTitle>,
  pub opus: Option<Opus>,
}

/// Works are optionally identified by number and title.
/// 
/// The [Work] element also may indicate a link to the [Opus] document that composes multiple scores into a collection.
#[derive(Debug, PartialEq, Eq, ElementDeserialize, ElementSerialize)]
pub struct Work {
  /// Element-specific attributes
  pub attributes: (),
  #[flatten]
  /// Element-specific content
  pub content: WorkContents,
}
