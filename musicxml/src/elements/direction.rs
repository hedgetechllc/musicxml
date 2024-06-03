use super::{DirectionType, Footnote, Level, Listening, Offset, Sound, Staff, Voice};
use crate::datatypes::{AboveBelow, Id, SystemRelation, YesNo};
use musicxml_internal::*;
use musicxml_macros::*;

/// Attributes pertaining to the [Direction] element.
#[derive(Debug, Default, PartialEq, Eq, AttributeDeserialize, AttributeSerialize)]
pub struct DirectionAttributes {
  /// Changes the default_x position of a [Direction]. It indicates that the left-hand side of the direction
  /// is aligned with the left-hand side of the time signature. If no time signature is present, the direction
  /// is aligned with the left-hand side of the first music notational element in the measure. If a `default_x`,
  /// `justify`, or `halign` attribute is present, it overrides this attribute.
  pub directive: Option<YesNo>,
  /// Specifies an ID that is unique to the entire document.
  pub id: Option<Id>,
  /// Indicates whether something is above or below another element, such as a note or a notation.
  pub placement: Option<AboveBelow>,
  /// Distinguishes elements that are associated with a system rather than the particular part where the element appears.
  pub system: Option<SystemRelation>,
}

#[derive(Debug, Default, PartialEq, Eq, ContentDeserialize, ContentSerialize)]
pub struct DirectionContents {
  pub direction_type: Vec<DirectionType>,
  pub offset: Option<Offset>,
  pub footnote: Option<Footnote>,
  pub level: Option<Level>,
  pub voice: Option<Voice>,
  pub staff: Option<Staff>,
  pub sound: Option<Sound>,
  pub listening: Option<Listening>,
}

/// A [Direction] is a musical indication that is not necessarily attached to a specific note.
///
/// Two or more may be combined to indicate words followed by the start of a dashed line, the end of a wedge followed by dynamics, etc.
/// For applications where a specific direction is indeed attached to a specific note, the [Direction] element can be associated with the
/// first [Note][super::Note] element that follows it in score order that is not in a different voice.
///
/// By default, a series of [DirectionType] elements and a series of child elements of a [DirectionType] within a single [Direction] element follow one
/// another in sequence visually. For a series of [DirectionType] children, non-positional formatting attributes are carried over from the previous
/// element by default.
#[derive(Debug, PartialEq, Eq, ElementDeserialize, ElementSerialize)]
pub struct Direction {
  /// Element-specific attributes
  pub attributes: DirectionAttributes,
  #[flatten]
  /// Element-specific content
  pub content: DirectionContents,
}
