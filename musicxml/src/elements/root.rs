use super::{RootAlter, RootStep};
use musicxml_internal::*;
use musicxml_macros::*;

#[derive(Debug, PartialEq, Eq, ContentDeserialize, ContentSerialize)]
pub struct RootContents {
  pub root_step: RootStep,
  pub root_alter: Option<RootAlter>,
}

/// The [Root] element indicates a pitch like C, D, E vs. a scale degree like 1, 2, 3.
///
/// It is used with chord symbols in popular music. The [Root] element has a [RootStep] and optional [RootAlter] element similar to the
/// [Step][super::Step] and [Alter][super::Alter] elements, but renamed to distinguish the different musical meanings.
#[derive(Debug, PartialEq, Eq, ElementDeserialize, ElementSerialize)]
pub struct Root {
  /// Element-specific attributes
  pub attributes: (),
  #[flatten]
  /// Element-specific content
  pub content: RootContents,
}
