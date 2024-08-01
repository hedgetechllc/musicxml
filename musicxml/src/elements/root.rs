use super::{RootAlter, RootStep};
use alloc::{string::String, vec::Vec};
use musicxml_internal::*;
use musicxml_macros::*;

/// Contents of the [Root] element.
#[derive(Debug, PartialEq, Eq, ContentDeserialize, ContentSerialize)]
pub struct RootContents {
  /// The [RootStep] element specifies the pitch like C, D, E vs. a scale degree like 1, 2, 3.
  pub root_step: RootStep,
  /// The [RootAlter] element specifies the chromatic alteration of the root of the chord.
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
