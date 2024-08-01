use super::{Opus, WorkNumber, WorkTitle};
use alloc::{string::String, vec::Vec};
use musicxml_internal::*;
use musicxml_macros::*;

/// Contents of the [Work] element.
#[derive(Debug, Default, PartialEq, Eq, ContentDeserialize, ContentSerialize)]
pub struct WorkContents {
  /// The [WorkNumber] element specifies the number of the work.
  pub work_number: Option<WorkNumber>,
  /// The [WorkTitle] element specifies the title of the work.
  pub work_title: Option<WorkTitle>,
  /// The [Opus] element specifies the opus number of the work.
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
