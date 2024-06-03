use super::{TupletDot, TupletNumber, TupletType};
use musicxml_internal::*;
use musicxml_macros::*;

#[derive(Debug, Default, PartialEq, Eq, ContentDeserialize, ContentSerialize)]
pub struct TupletActualContents {
  pub tuplet_number: Option<TupletNumber>,
  pub tuplet_type: Option<TupletType>,
  pub tuplet_dot: Vec<TupletDot>,
}

/// The [TupletActual] element provides optional full control over how the actual part of the [Tuplet][super::Tuplet] is displayed,
/// including number and note type (with dots).
/// 
/// If any of these elements are absent, their values are based on the [TimeModification][super::TimeModification] element.
#[derive(Debug, PartialEq, Eq, ElementDeserialize, ElementSerialize)]
#[rename("tuplet-actual")]
pub struct TupletActual {
  /// Element-specific attributes
  pub attributes: (),
  #[flatten]
  /// Element-specific content
  pub content: TupletActualContents,
}
