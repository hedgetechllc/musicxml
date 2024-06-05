use crate::datatypes::{BeamLevel, BeamValue, Color, Fan, Id, YesNo};
use musicxml_internal::*;
use musicxml_macros::*;

/// Attributes pertaining to the [Beam] element.
#[derive(Debug, Default, PartialEq, Eq, AttributeDeserialize, AttributeSerialize)]
pub struct BeamAttributes {
  /// Indicates the color of an element.
  pub color: Option<Color>,
  /// Beams that have a begin value may also have a fan attribute to indicate accelerandos and ritardandos using fanned beams.
  /// The `fan` attribute may also be used with a continue value if the fanning direction changes on that note.
  /// The value is none if not specified.
  pub fan: Option<Fan>,
  /// Specifies an ID that is unique to the entire document.
  pub id: Option<Id>,
  /// Indicates eighth note through 1024th note beams using number values 1 thru 8 respectively. The default value is 1.
  ///
  /// Note that this attribute does not distinguish sets of beams that overlap, as it does for [Slur][super::Slur] and other elements.
  /// Beaming groups are distinguished by being in different voices and/or the presence or absence of [Grace][super::Grace] and [Cue][super::Cue] elements.
  pub number: Option<BeamLevel>,
  /// Deprecated as of Version 3.0. Formerly used for tremolos, it needs to be specified with a "yes" value for each [Beam] using it.
  pub repeater: Option<YesNo>,
}

/// Beam values include begin, continue, end, forward hook, and backward hook.
///
/// Each beam in a note is represented with a separate [Beam] element with a different `number` attribute,
/// starting with the eighth note beam using a value of 1:
///
/// ![Beam](https://hedgetechllc.github.io/musicxml/musicxml/elements/beam.png)
#[derive(Debug, PartialEq, Eq, ElementDeserialize, ElementSerialize)]
pub struct Beam {
  /// Element-specific attributes
  pub attributes: BeamAttributes,
  /// Element-specific content
  pub content: BeamValue,
}
