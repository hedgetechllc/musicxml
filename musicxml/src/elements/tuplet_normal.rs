use super::{TupletDot, TupletNumber, TupletType};
use musicxml_internal::*;
use musicxml_macros::*;

/// Contents of the [TupletNormal] element.
#[derive(Debug, Default, PartialEq, Eq, ContentDeserialize, ContentSerialize)]
pub struct TupletNormalContents {
  /// The [TupletNumber] element specifies the number of notes in the tuplet.
  pub tuplet_number: Option<TupletNumber>,
  /// The [TupletType] element specifies the note type of the tuplet.
  pub tuplet_type: Option<TupletType>,
  /// The [TupletDot] element specifies the presence of a dot on the note type of the tuplet.
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
