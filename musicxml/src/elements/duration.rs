use crate::datatypes::PositiveDivisions;
use musicxml_internal::*;
use musicxml_macros::*;

/// The [Duration] element represents the intended duration of a note as a positive number specified in [Divisions][super::Divisions] units.
///
/// Differences in duration specific to an interpretation or performance should be represented using the [Note][super::Note] element's `attack` and `release` attributes.
///
/// The [Duration] element moves the musical position when used in [Backup][super::Backup] elements, [Forward][super::Forward] elements, and
/// [Note][super::Note] elements that do not contain a [Chord][super::Chord] child element.
#[derive(Debug, PartialEq, Eq, ElementDeserialize, ElementSerialize)]
pub struct Duration {
  /// Element-specific attributes
  pub attributes: (),
  /// Element-specific content
  pub content: PositiveDivisions,
}
