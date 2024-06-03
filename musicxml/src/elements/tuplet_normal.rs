use super::{TupletDot, TupletNumber, TupletType};
use musicxml_internal::*;
use musicxml_macros::*;

#[derive(Debug, Default, PartialEq, Eq, ContentDeserialize, ContentSerialize)]
pub struct TupletNormalContents {
  pub tuplet_number: Option<TupletNumber>,
  pub tuplet_type: Option<TupletType>,
  pub tuplet_dot: Vec<TupletDot>,
}

/// The [TupletNormal] element provides optional full control over how the normal part of the [Tuplet][super::Tuplet] is displayed,
/// including number and note type (with dots).
/// 
/// If any of these elements are absent, their values are based on the [TimeModification][super::TimeModification] element.
#[derive(Debug, PartialEq, Eq, ElementDeserialize, ElementSerialize)]
#[rename("tuplet-normal")]
pub struct TupletNormal {
  /// Element-specific attributes
  pub attributes: (),
  #[flatten]
  /// Element-specific content
  pub content: TupletNormalContents,
}
