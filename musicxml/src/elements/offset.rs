use crate::datatypes::{Divisions, YesNo};
use musicxml_internal::*;
use musicxml_macros::*;

/// Attributes pertaining to the [Offset] element.
#[derive(Debug, Default, PartialEq, Eq, AttributeDeserialize, AttributeSerialize)]
pub struct OffsetAttributes {
  /// The offset affects the visual appearance of the direction. If the `sound` attribute is yes,
  /// then the offset affects playback and listening too. If it is no, then any [Sound][super::Sound] or
  /// [Listening][super::Listening] associated with the [Direction][super::Direction] takes effect at the
  /// current location. It is no if not specified for compatibility with earlier MusicXML versions.
  pub sound: Option<YesNo>,
}

/// An [Offset] is represented in terms of divisions, and indicates where the direction will appear relative to the current musical location.
/// 
/// The current musical location is always within the current measure, even at the end of a measure. If an element within a [Direction][super::Direction]
/// includes a `default_x` attribute, the [Offset] value will be ignored when determining the appearance of that element.
#[derive(Debug, PartialEq, Eq, ElementDeserialize, ElementSerialize)]
pub struct Offset {
  /// Element-specific attributes
  pub attributes: OffsetAttributes,
  /// Element-specific content
  pub content: Divisions,
}
