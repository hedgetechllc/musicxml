use crate::datatypes::{Divisions, Percent, YesNo};
use alloc::{string::String, vec::Vec};
use musicxml_internal::*;
use musicxml_macros::*;

/// Attributes pertaining to the [Grace] element.
#[derive(Debug, Default, PartialEq, Eq, AttributeDeserialize, AttributeSerialize)]
pub struct GraceAttributes {
  /// Indicates to make time, not steal time, for grace note playback.
  /// The units are in real-time divisions for the grace note.
  pub make_time: Option<Divisions>,
  /// The value is yes for slashed grace notes and no if no slash is present.
  pub slash: Option<YesNo>,
  /// Indicates the percentage of time to steal from the following note for the grace note playback, as for appoggiaturas.
  pub steal_time_following: Option<Percent>,
  /// The `steal_time_previous` attribute indicates the percentage of time to steal from the previous note for the grace note playback.
  pub steal_time_previous: Option<Percent>,
}

/// The [Grace] element indicates the presence of a grace note.
///
/// ![Grace](https://hedgetechllc.github.io/musicxml/musicxml/elements/grace.png)
#[derive(Debug, PartialEq, Eq, ElementDeserialize, ElementSerialize)]
pub struct Grace {
  /// Element-specific attributes
  pub attributes: GraceAttributes,
  /// Element-specific content
  pub content: (),
}
